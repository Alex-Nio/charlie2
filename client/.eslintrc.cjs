module.exports = {
  root: true,
  env: {
    node: true,
  },
  plugins: ['prettier'],
  extends: [
    'plugin:vue/vue3-essential',
    'eslint:recommended',
    'plugin:prettier/recommended',
  ],
  rules: {
    // СТИЛЬ РАЗРЫВА СТРОК
    'linebreak-style': ['error', 'unix'], // Устанавливает стиль разрыва строки в формат Windows (CRLF).

    // ОТСУТСТВИЕ CONSOLE.LOG
    'no-console': 'off', // Запрещает использование console.log в коде.

    // ОТСУТСТВИЕ DEBUGGER
    'no-debugger': 'error', // Запрещает использование оператора debugger в коде.

    // СКОБКИ В СТРЕЛОЧНЫХ ФУНКЦИЯХ
    'arrow-parens': ['error', 'always', { requireForBlockBody: true }], // Требует использование скобок в стрелочных функциях только при необходимости.

    // ОТКЛЮЧЕНИЕ УНАРНЫХ ОПЕРАТОРОВ ++ И --
    'no-plusplus': 'off', // Разрешает использование унарных операторов ++ и --.

    // ОТКЛЮЧЕНИЕ ОБЯЗАТЕЛЬНОГО ВЫЗОВА SUPER() В КОНСТРУКТОРАХ
    'constructor-super': 'off', // Отключает требование вызова super() в конструкторах производных классов.

    // ГРУППИРОВКА СЛОЖНЫХ ВЫРАЖЕНИЙ
    'no-mixed-operators': [
      'error',
      {
        groups: [
          ['+', '-', '*', '/', '%', '**'],
          ['&', '|', '^', '~', '<<', '>>', '>>>'],
          ['==', '!=', '===', '!==', '>', '>=', '<', '<='],
          ['&&', '||'],
          ['in', 'instanceof'],
        ],
        allowSamePrecedence: true,
      },
    ], // Заключает сложные выражения в круглые скобки для ясности.

    // ОТКЛЮЧЕНИЕ ПРАВИЛА О РАСШИРЕНИИ ФАЙЛОВ В ПУТИ ИМПОРТА
    'import/extensions': 'off', // Отключает правило обязательного указания расширения файла в пути импорта.

    // ОТКЛЮЧЕНИЕ ПРАВИЛА О ПРЕДПОЧТЕНИИ ЭКСПОРТА ПО УМОЛЧАНИЮ
    'import/prefer-default-export': 'off', // Отключает предпочтение экспорта по умолчанию.

    // ОТСУТСТВИЕ НЕИСПОЛЬЗУЕМЫХ ВЫРАЖЕНИЙ
    'no-unused-expressions': 'error', // Запрещает неиспользуемые выражения.

    // ОТКЛЮЧЕНИЕ ЗАПРЕТА ПЕРЕНАЗНАЧЕНИЯ ПАРАМЕТРОВ
    'no-param-reassign': 'off', // Разрешает переназначение параметров функции.

    // ДЕСТРУКТУРИЗАЦИЯ
    'prefer-destructuring': [
      'error',
      {
        array: true, // Требует деструктуризацию массивов.
        object: true, // Требует деструктуризацию объектов.
      },
      {
        enforceForRenamedProperties: false,
      },
    ],

    // ЗАПРЕТ ПОБИТОВЫХ ОПЕРАТОРОВ, КРОМЕ ~
    'no-bitwise': [
      'error',
      {
        allow: ['~'],
      },
    ],

    // ЗАПРЕТ НЕИСПОЛЬЗУЕМЫХ ПЕРЕМЕННЫХ
    'no-unused-vars': [
      'error',
      {
        argsIgnorePattern: '^_', // Игнорировать неиспользуемые переменные, начинающиеся с символа '_'.
      },
    ],

    // МАКСИМАЛЬНАЯ ДЛИНА СТРОКИ
    'max-len': [
      'error',
      {
        code: 120, // Устанавливает максимальную длину строки кода в 120 символов.
      },
    ],

    // РАЗРЫВ СТРОК ПОКАЗАНИЯ ФИГУРНЫХ СКОБОК
    'object-curly-newline': [
      'error',
      {
        ObjectExpression: {
          multiline: true,
          consistent: true,
        },
        ObjectPattern: {
          multiline: true,
          consistent: true,
        },
      },
    ], // Применяет согласованные разрывы строк после открытия и перед закрытием фигурных скобок.

    // ПУСТАЯ СТРОКА МЕЖДУ ЧЛЕНАМИ КЛАССАМИ
    'lines-between-class-members': [
      'error',
      'always',
      {
        exceptAfterSingleLine: true,
      },
    ], // Требует пустую строку между членами класса.

    // ПОРЯДОК АТРИБУТОВ В ТЕГАХ VUE
    'vue/attributes-order': [
      'error',
      {
        order: [
          'DEFINITION',
          'LIST_RENDERING',
          'CONDITIONALS',
          'RENDER_MODIFIERS',
          'GLOBAL',
          ['UNIQUE', 'SLOT'],
          'TWO_WAY_BINDING',
          'OTHER_DIRECTIVES',
          'OTHER_ATTR',
          'EVENTS',
          'CONTENT',
        ],
        alphabetical: false,
      },
    ],

    // МАКСИМАЛЬНОЕ КОЛИЧЕСТВО АТРИБУТОВ В ТЕГАХ VUE
    'vue/max-attributes-per-line': [
      'error',
      {
        singleline: {
          max: 1,
        },
        multiline: {
          max: 1,
        },
      },
    ],

    // HTML-САМОЗАКРЫВАЮЩИЕСЯ ТЕГИ
    'vue/html-self-closing': [
      'error',
      {
        // Правила для тегов HTML
        html: {
          // Запрещаем использование самозакрывающихся тегов для тегов без содержимого (void elements)
          void: 'always',
          // Всегда использовать самозакрывающиеся теги для обычных тегов (не void, с содержимым)
          normal: 'always',
          // Всегда использовать самозакрывающиеся теги для компонентов Vue
          component: 'always',
        },
        // Правила для тегов SVG
        svg: 'always',
        // Правила для тегов MathML
        math: 'always',
      },
    ],

    // ОТСТУПЫ В HTML ТЕГАХ
    'vue/html-indent': [
      'error',
      2,
      {
        attribute: 1,
        baseIndent: 1,
        closeBracket: 0,
        alignAttributesVertically: true,
        ignores: [],
      },
    ],

    // ПРАВИЛА ИМЕНОВАНИЯ КОМПОНЕНТОВ В ТЕГАХ VUE
    'vue/component-name-in-template-casing': [
      'error',
      'PascalCase',
      {
        registeredComponentsOnly: true,
      },
    ],

    // ЗАПРЕТ НЕПРАВИЛЬНЫХ ПРОБЕЛОВ В ТЕКСТЕ VUE
    'vue/no-irregular-whitespace': [
      'error',
      {
        skipStrings: true,
        skipComments: false,
        skipRegExps: false,
        skipTemplates: false,
        skipHTMLAttributeValues: false,
        skipHTMLTextContents: false,
      },
    ],

    // ПРАВИЛА ИМЕНОВАНИЯ КОМПОНЕНТОВ В ОПРЕДЕЛЕНИИ КОМПОНЕНТОВ VUE
    'vue/component-definition-name-casing': ['error', 'kebab-case'],

    // СООТВЕТСТВИЕ ИМЕН ФАЙЛОВ КОМПОНЕНТОВ
    'vue/match-component-file-name': [
      'error',
      {
        extensions: ['vue'],
        shouldMatchCase: false,
      },
    ],

    // ЗАПРЕТ ДУБЛИРОВАНИЯ КЛЮЧЕЙ В ТЕГАХ VUE
    'vue/no-dupe-keys': [
      'error',
      {
        groups: [],
      },
    ],

    // ПОРЯДОК ЭЛЕМЕНТОВ В ОПРЕДЕЛЕНИИ КОМПОНЕНТОВ VUE
    'vue/order-in-components': [
      'error',
      {
        order: [
          'el',
          'name',
          'key',
          'parent',
          'functional',
          ['delimiters', 'comments'],
          ['components', 'directives', 'filters'],
          'extends',
          'mixins',
          ['provide', 'inject'],
          'ROUTER_GUARDS',
          'layout',
          'middleware',
          'validate',
          'scrollToTop',
          'transition',
          'loading',
          'inheritAttrs',
          'model',
          ['props', 'propsData'],
          'emits',
          'setup',
          'asyncData',
          'data',
          'fetch',
          'head',
          'computed',
          'watch',
          'watchQuery',
          'LIFECYCLE_HOOKS',
          'methods',
          ['template', 'render'],
          'renderError',
        ],
      },
    ],
    // ЗАПЯТАЯ В КОНЦЕ ОБЪЕКТОВ И МАССИВОВ
    'comma-dangle': 'off',
    'comma-dangle': [
      'error',
      {
        arrays: 'never',
        objects: 'always-multiline',
        imports: 'never',
        exports: 'never',
        functions: 'never',
      },
    ],
  },
};
