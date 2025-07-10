import antfu from '@antfu/eslint-config'

export default antfu({
  typescript: true,
  vue: true,
  ignores: [
    'node_modules',
    './ws/schemas/src/gen.ts',
    './ws/core/gen/**/*',
    './ws/ui/src/shared/tp/**/*',
  ],
  rules: {
    'ts/no-namespace': 'off',
    'ts/no-redeclare': 'off',
    'vue/attribute-hyphenation': ['error', 'never'],
    'vue/v-on-event-hyphenation': ['error', 'never'],
  },
})
