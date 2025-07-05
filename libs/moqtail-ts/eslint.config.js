// @ts-check

import rootConfig from '../../eslint.config.mjs'

export default [
  ...rootConfig,
  {
    rules: {
      '@typescript-eslint/no-unused-vars': [
        'error',
        {
          argsIgnorePattern: '^_',
          varsIgnorePattern: '^_',
          caughtErrorsIgnorePattern: '^_',
        },
      ],
      // TODO: switch to export const Type = { .. } as const
      '@typescript-eslint/no-namespace': 'off',
    },
  },
  {
    // Allow 'any' in Promise interface implementations
    files: ['src/client/request/*.ts'],
    rules: {
      '@typescript-eslint/no-explicit-any': 'off',
    },
  },
]
