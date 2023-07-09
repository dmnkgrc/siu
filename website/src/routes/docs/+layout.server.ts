import fs from 'node:fs';

import type { PageServerLoad } from './$types';
import type { Node } from './types';

const base = './docs';

const getNodes = (options: { path: string; requestedPath: string }): Node[] => {
	const structure: Node[] = [];
	const currentPath = `${base}/${options.path}`;
	for (const subdir of fs.readdirSync(currentPath)) {
		const newSubPath = options.path ? `${options.path}/${subdir}` : subdir;
		const newPath = `${base}/${newSubPath}`;
		const url = newPath.replace('.', '').replace('.mdx', '');
		const name = subdir.replace('.mdx', '').replaceAll('-', ' ');

		if (fs.statSync(newPath).isDirectory()) {
			structure.push({
				name,
				url,
				children: getNodes({ path: newSubPath, requestedPath: options.requestedPath })
			});
			continue;
		}
		structure.push({
			name,
			url,
			isActive: newSubPath === `${options.requestedPath}.mdx`
		});
	}
	return structure;
};

export const prerender = true;

export const load: PageServerLoad = async ({ params }) => {
	const crumbs = params.slug.split('/').map((part) => part.replaceAll('-', ' '));
	return {
		crumbs,
		structure: getNodes({ path: '', requestedPath: params.slug })
	};
};
