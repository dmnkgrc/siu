import fs from 'node:fs'
import { unified } from 'unified'
import remarkParse from 'remark-parse'
import remarkFrontmatter from 'remark-frontmatter'
import remarkParseFrontmatter from 'remark-parse-frontmatter'
import remarkRehype from 'remark-rehype'
import rehypeStringify from 'rehype-stringify'
import rehypeHighlight from 'rehype-highlight'
import { z } from 'zod'

const base = '../docs'

import type { PageServerLoad } from './$types'
import { error } from '@sveltejs/kit'

const parseFile = async (path: string) => {
  const file = await unified()
    .use(remarkParse)
    .use(remarkFrontmatter, ['yaml'])
    .use(remarkParseFrontmatter)
    .use(remarkRehype)
    .use(rehypeHighlight)
    .use(rehypeStringify)
    .process(fs.readFileSync(path, 'utf8'))
  const frontmatter = z.object({
     title: z.string(),
  }).parse(file.data.frontmatter)
  return {
    frontmatter: frontmatter,
    raw: fs.readFileSync(path, 'utf8'),
    content: file.toString(),
  }
}

export const load: PageServerLoad = async ({ params }) => {
  const path = `${base}/${params.slug}.md`;
  const fileExists = fs.statSync(path, { throwIfNoEntry: false })?.isFile() ?? false
  if (fileExists) {
    return {
      page: await parseFile(path)
    }
  }
  throw error(404)
}
