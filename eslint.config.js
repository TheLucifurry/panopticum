import antfu from '@antfu/eslint-config'

export default antfu({
  typescript: true,
  vue: true,
  ignores: [
    'node_modules',
    './ws/schemas/src/gen.ts',
    './ws/core/gen/**/*',
  ],
  rules: {
    'ts/no-namespace': 'off',
    'ts/no-redeclare': 'off',
  },
})
