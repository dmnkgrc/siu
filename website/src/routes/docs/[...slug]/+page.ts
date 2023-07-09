import { error } from '@sveltejs/kit';

import type { PageLoad } from './$types';

const base = '../../../../docs';

export const load: PageLoad = async ({ params }) => {
	const filePath = `${base}/${params.slug}.mdx`;
	try {
		const doc = await import(/* @vite-ignore */ filePath);
		return {
			metadata: doc.metadata,
			content: doc.default
		};
	} catch (e) {
		console.log(e);
		throw error(404);
	}
};
