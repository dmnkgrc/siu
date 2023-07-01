import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";

import svelte from "@astrojs/svelte";
import tailwind from "@astrojs/tailwind";

// https://astro.build/config
export default defineConfig({
  integrations: [
    starlight({
      title: "smu",
      defaultLocale: "root",
      locales: {
        root: {
          lang: "en",
          label: "English",
        },
      },
      social: {
        github: "https://github.com/dmnkgrc/smu",
      },
      sidebar: [
        {
          label: "Guides",
          autogenerate: {
            directory: "guides",
          },
        },
        {
          label: "Reference",
          autogenerate: {
            directory: "reference",
          },
        },
      ],
    }),
    svelte(),
    tailwind(),
  ],
  image: {
    service: {
      entrypoint: "astro/assets/services/sharp",
    },
  },
});
