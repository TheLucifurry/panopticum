import antfu from '@antfu/eslint-config'

export default antfu({
  ignores: [
    '**/*.d.ts', // FIXME: disable while "Unexpected token module" appears
  ],
})
