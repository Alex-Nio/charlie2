import asyncio
import os
import time
import pygame
import numpy as np
import librosa
from pydub import AudioSegment
import subprocess
import yt_dlp
import soundfile as sf
from fairseq import checkpoint_utils
import numba as nb
import gradio as gr
import edge_tts
import torch
from spleeter.separator import Separator
from config import Config
from lib.infer_pack.models import (
    SynthesizerTrnMs256NSFsid,
    SynthesizerTrnMs256NSFsid_nono,
    SynthesizerTrnMs768NSFsid,
    SynthesizerTrnMs768NSFsid_nono,
)
from rmvpe import RMVPE
from vc_infer_pipeline import VC
from dotenv import load_dotenv  # Import the library

# Load environment variables from .env file
load_dotenv()

pygame.init()

target_sample_rate = 48000
edge_output_filename = "edge_output.mp3"

voice_model_root = os.getenv("VOICE_MODEL_ROOT")
hubert_model_root = os.getenv("HUBERT_MODEL_ROOT")
rmvpe_model_root = os.getenv("RMVPE_MODEL_ROOT")

limitation = os.getenv("SYSTEM") == "spaces"


def load_hubert():
    print("[+] Loading hubert model...")

    global hubert_model

    models, _, _ = checkpoint_utils.load_model_ensemble_and_task(
        [hubert_model_root],
        suffix="",
    )

    hubert_model = models[0]
    hubert_model = hubert_model.to(config.device)

    if config.is_half:
        hubert_model = hubert_model.half()
    else:
        hubert_model = hubert_model.float()
    return hubert_model.eval()


@nb.jit(nopython=True, parallel=True, fastmath=True)
def process_audio(audio, tgt_sr, filter_radius, rms_mix_rate):
    for i in nb.prange(len(audio)):
        audio[i] = audio[i] * 2.0
    return audio


def play_audio(file_path, play=True):
    try:
        pygame.mixer.music.load(file_path)

        if play:
            pygame.mixer.music.play()

            while pygame.mixer.music.get_busy():
                pygame.time.Clock().tick(10)

    except pygame.error as e:
        print(f"Ошибка воспроизведения аудио: {e}")
    finally:
        if play:
            pygame.quit()


async def tts_process_async(
    model_name,
    f0_key,
    f0_method,
    volume,
    vc_gain,
    is_normal,
    index_rate,
    protect,
    tts_text,
    tts_voice,
    speed,
    filter_radius,
    resample_sr,
    rms_mix_rate,
    audio_format,
    bitrate,
):
    # Определение пути к файлу
    voice_path = f"tts_voice.{audio_format}"

    # Удаление файла, если он существует
    if os.path.exists(voice_path):
        os.remove(voice_path)
        print(f"Удален существующий файл: {voice_path}")

    tgt_sr, net_g, vc, version, index_file, if_f0 = model_data(model_name)

    print(" ")
    print("text:")
    print(tts_text)
    print(f"voice: {tts_voice}")

    t0 = time.time()

    speed_str = f"+{speed}%" if speed >= 0 else f"{speed}%"
    await edge_tts.Communicate(
        tts_text, "-".join(tts_voice.split("-")[:-1]), rate=speed_str
    ).save(edge_output_filename)

    print("selected TTS")

    t1 = time.time()
    edge_time = t1 - t0
    audio, sr = librosa.load(edge_output_filename, sr=16000, mono=True)
    duration = len(audio) / sr

    print(f"Audio duration: {duration}s")

    if limitation and duration >= 30:
        print("Error: Audio too long")
        return (
            f"Audio should be less than 30 seconds in this huggingface space, but got {duration}s.",
            edge_output_filename,
            None,
        )

    f0_key = int(f0_key)

    if not hubert_model:
        load_hubert()

    vc.model_rmvpe = rmvpe_model
    times = [0, 0, 0]

    audio = process_audio(audio, tgt_sr, filter_radius, rms_mix_rate)

    audio_opt = vc.pipeline(
        hubert_model,
        net_g,
        0,
        audio,
        edge_output_filename,
        times,
        f0_key,
        f0_method,
        index_file,
        index_rate,
        if_f0,
        filter_radius,
        tgt_sr,
        resample_sr,
        rms_mix_rate,
        version,
        protect,
        None,
        volume,
    )

    if tgt_sr != resample_sr >= 16000:
        tgt_sr = resample_sr

    info = f"Successfully! Time of processing: edge-tts: {edge_time}s, npy: {times[0]}s, f0: {times[1]}s, infer: {times[2]}s"

    print(info)

    audio_data_bytes = np.array(audio_opt, dtype=np.int16).tobytes()
    audio_segment = AudioSegment(
        audio_data_bytes, frame_rate=tgt_sr, sample_width=2, channels=1
    )

    if is_normal:
        audio_segment = audio_segment.normalize()

    audio_segment.export(voice_path, format=audio_format, bitrate=str(bitrate) + "k")
    voice_audio = AudioSegment.from_file(voice_path)
    resampled_voice_audio = voice_audio.set_channels(2).set_frame_rate(
        target_sample_rate
    )
    resampled_voice_audio = resampled_voice_audio + (vc_gain)
    resampled_voice_audio.export(voice_path, format=audio_format)

    play_audio(voice_path)

    return (
        info,
        edge_output_filename,
        voice_path,
    )


def model_data(model_name):
    pth_files = [
        os.path.join(voice_model_root, model_name, f)
        for f in os.listdir(os.path.join(voice_model_root, model_name))
        if f.endswith(".pth")
    ]

    if len(pth_files) == 0:
        raise ValueError(f"No pth file found in {voice_model_root}/{model_name}")

    pth_path = pth_files[0]

    print(f"Loading {pth_path}")

    cpt = torch.load(pth_path, map_location="cpu")
    tgt_sr = cpt["config"][-1]
    cpt["config"][-3] = cpt["weight"]["emb_g.weight"].shape[0]
    if_f0 = cpt.get("f0", 1)
    version = cpt.get("version", "v1")

    if version == "v1":
        if if_f0 == 1:
            net_g = SynthesizerTrnMs256NSFsid(*cpt["config"], is_half=config.is_half)
        else:
            net_g = SynthesizerTrnMs256NSFsid_nono(*cpt["config"])
    elif version == "v2":
        if if_f0 == 1:
            net_g = SynthesizerTrnMs768NSFsid(*cpt["config"], is_half=config.is_half)
        else:
            net_g = SynthesizerTrnMs768NSFsid_nono(*cpt["config"])
    else:
        raise ValueError("Unknown version")

    del net_g.enc_q

    net_g.load_state_dict(cpt["weight"], strict=False)

    print("Model loaded")

    net_g.eval().to(config.device)

    if config.is_half:
        net_g = net_g.half()
    else:
        net_g = net_g.float()

    vc = VC(tgt_sr, config)

    index_files = [
        os.path.join(voice_model_root, model_name, f)
        for f in os.listdir(os.path.join(voice_model_root, model_name))
        if f.endswith(".index")
    ]

    if len(index_files) == 0:
        print("No index file found")
        index_file = ""
    else:
        index_file = index_files[0]
        print(f"Index file found: {index_file}")
    return tgt_sr, net_g, vc, version, index_file, if_f0


async def main_async():
    config = Config()
    text_from_command_line = config.text
    await tts_process_async(
        "charlie",
        0,
        "rmvpe",
        100,
        0,
        True,
        0.5,
        0.5,
        text_from_command_line,
        "ru-RU-DmitryNeural-Male",
        0,
        3,
        0,
        0.25,
        "mp3",
        128,
    )


def main():
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main_async())


if __name__ == "__main__":
    config = Config()

    hubert_model = load_hubert()
    print("[+] Hubert model loaded.")
    print("[+] Loading rmvpe model...")

    rmvpe_model = RMVPE(rmvpe_model_root, config.is_half, config.device)
    print("rmvpe model loaded.")

    tgt_sr, net_g, vc, version, index_file, if_f0 = model_data(
        "charlie"
    )  # Загрузка модели "charlie"

    print("Models loaded. Starting main...")
    main()
