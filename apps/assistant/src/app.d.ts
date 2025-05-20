// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}

	interface Window {
		__TAURI_INTERNALS__?: any; // Use 'any' for simplicity, or define a more specific type if known
	}
}

export {};
