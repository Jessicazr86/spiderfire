declare class AbortController {
	constructor();

	get signal(): AbortSignal;
	abort(reason?: any): void;
}

declare class AbortSignal {
	static abort(reason?: any): AbortSignal;
	static timeout(time: number): AbortSignal;

	get aborted(): boolean;
	get reason(): any;

	throwIfAborted(): void;
}
