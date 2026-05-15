export type Config = {
	server: {
		port: number;
	};
	bluetooth: {
		adapter: string;
	};
	devices: {
		name: string;
		address: string;
		handle: number;
		order: string;
	}[];
};
