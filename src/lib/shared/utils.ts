export function notNull<T>(value: T | null): value is T {
	return value !== null;
}

export function notNullOrUndefined<T>(value: T | null | undefined): value is T {
	return value !== null && value !== undefined;
}
