declare module "http" {
	export type Header = string | string[];
	export type HeaderEntries = [string, string][];
	export interface HeadersObject {
		[key: string]: Header,
	}
	export type HeadersInit = Headers | HeaderEntries | HeadersObject;

	export type Body = string | String | ArrayBuffer | TypedArray | DataView;

	export interface RequestOptions {
		auth?: string,
		setHost?: boolean,

		client?: ClientRequestOptions,
		redirect?: Redirect,
		signal?: AbortSignal,

		headers?: HeadersInit,
		body?: Body,
	}

	export type RequestBuilderOptions = RequestOptions & {
		method?: string,
	};

	export interface ClientOptions {
		keepAlive?: boolean,
		keepAliveTimeout?: number,
		maxIdleSockets?: number,
		retryCancelled?: boolean,
	}

	export type ClientRequestOptions = undefined | boolean | Client;

	export type Redirect = "follow" | "error" | "manual";

	export function get(url: string, options?: RequestOptions): Promise<Response>;
	export function post(url: string, options?: RequestOptions): Promise<Response>;
	export function put(url: string, options?: RequestOptions): Promise<Response>;
	export function request(resource: string, method: string, options?: RequestOptions): Promise<Response>;
	export function request(resource: Request): Promise<Response>;

	export class Headers {
		constructor();
		constructor(headers: Headers);
		constructor(entries: HeaderEntries);
		constructor(object: HeadersObject);

		append(name: string, value: string);
		delete(name: string): boolean;
		get(name: string): Header | null;
		has(name: string): boolean;
		set(name: string, value: string);
	}

	export class Request {
		constructor(url: string, options?: RequestBuilderOptions);
		constructor(url: Request, options?: RequestBuilderOptions);
	}

	export class Response {
		get ok(): boolean;
		get status(): number;
		get statusText(): string;

		get bodyUsed(): boolean;
		get headers(): Headers;

		get url(): string;
		get redirected(): boolean;
		get locations(): string[];

		arrayBuffer(): Promise<ArrayBuffer>;
		text(): Promise<string>;
	}

	export class Client {
		constructor(options?: ClientOptions);
	}

	namespace Http {
		export {
			Body,
			RequestOptions,
			RequestBuilderOptions,

			get,
			post,
			put,
			request,

			Header,
			HeaderEntries,
			HeadersObject,

			Headers,
			Request,
			Response,

			Client,
		};
	}

	export default Http;
}
