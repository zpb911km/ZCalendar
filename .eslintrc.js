module.exports = {
  root: true,
  env: {
    browser: true,
    es2021: true,
    node: true,
  },
  extends: [
    'eslint:recommended',
    'plugin:vue/vue3-essential', // 或者使用 vue3-recommended 以获得更严格的检查
    '@vue/eslint-config-typescript',
    'prettier', // 添加 prettier 推荐配置
  ],
  plugins: [
    'vue',
    '@vue'
  ],
  parserOptions: {
    ecmaVersion: 'latest',
    parser: '@typescript-eslint/parser',
    sourceType: 'module',
  },
  rules: {
    'vue/multi-word-component-names': 'off', // 关闭多词组件名检查，因为项目中可能有单词组件名
  },
}