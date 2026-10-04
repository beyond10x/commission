import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';

/**
 * Hand-written concepts and guides first, then the reference pages `commission-docs` generates
 * from the ESS specification, then where the project stands.
 */
const sidebars: SidebarsConfig = {
  docsSidebar: [
    {
      type: 'category',
      label: 'Start here',
      collapsed: false,
      items: ['index'],
    },
    {
      type: 'category',
      label: 'Concepts',
      collapsed: false,
      items: [
        'concepts/a-run',
        'concepts/rules',
        'concepts/executors',
        {
          type: 'category',
          label: 'Contract sketches',
          collapsed: false,
          items: [
            'concepts/contracts/frontier',
            'concepts/contracts/evidence',
            'concepts/contracts/executor',
          ],
        },
      ],
    },
    {
      type: 'category',
      label: 'Guides',
      collapsed: false,
      items: ['guides/build-from-source'],
    },
    {
      type: 'category',
      label: 'Reference (generated)',
      collapsed: false,
      items: ['reference/domain-model', 'reference/types'],
    },
    {
      type: 'category',
      label: 'Project status',
      collapsed: false,
      items: ['status'],
    },
  ],
};

export default sidebars;
