type CommonNodeOptions = {
	name: string;
	url: string;
};

export type Node = CommonNodeOptions &
	(
		| {
				children: Node[];
		  }
		| { isActive: boolean }
	);
