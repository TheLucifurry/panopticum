/*
 Generated by typeshare 1.13.2
*/

export interface IContentProviderManifestWrapper<T> {
	id: string;
	version: number;
	data: T;
}

export interface IContentProviderViews {
	main: string;
	search?: string;
}

export interface IContentProviderV0 {
	name: string;
	version: string;
	icons: Record<string, string>;
	description?: string;
	views: IContentProviderViews;
}

export type IContentProviderManifestV0 = IContentProviderManifestWrapper<IContentProviderV0>;

export type IContentProviderManifest = IContentProviderManifestV0;

export interface Paginated {
	current: number;
	size: number;
	total: number;
}

export type ContentNode = 
	| { type: "list", data: IContentList }
	| { type: "media", data: IContentMedia }
	| { type: "preview", data: IContentPreview };

export interface IContentList {
	name: string;
	page: Paginated;
	items: ContentNode[];
}

export interface IContentMedia {
	name: string;
	path: string;
	duration?: number;
	thumbnailPath?: string;
	mediaType: number;
	createdAt: string;
	isLocal: boolean;
	size?: string;
}

export enum ContentNodeType {
	List = "list",
	Media = "media",
	Preview = "preview",
}

export interface IContentPreview {
	type: ContentNodeType;
	pict?: string;
}

export enum ContentProviderViewKey {
	Main = "main",
	Search = "search",
}

export enum MediaType {
	Video = "Video",
	Audio = "Audio",
}

