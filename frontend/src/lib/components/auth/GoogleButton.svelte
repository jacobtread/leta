<!-- Google authentication button -->

<script lang="ts">
	import { PUBLIC_GOOGLE_CLIENT_ID } from "$env/static/public";

	enum FlowStage {
		/// Initial stage
		Initial,
		/// Authentication complete
		Complete,
		/// Conflict with existing account credentials
		Conflict
	}

	// Shorter type alias for google credential responses
	type CredentialResponse = google.accounts.id.CredentialResponse;

	// Output for the google auth token
	export let googleToken: string | null = null;

	let stage: FlowStage = FlowStage.Initial;

	let username: string = "";
	let password: string = "";

	// HTML parent for the google button to render within
	let googleButton: HTMLDivElement;

	let loading: boolean = false;
	let error: string | null = null;

	/**
	 * Handler triggered once the google client script has
	 * been loaded and executed
	 */
	function onScriptLoaded() {
		// Initialize Google ID context
		google.accounts.id.initialize({
			client_id: PUBLIC_GOOGLE_CLIENT_ID,
			callback: onGoogleIdentify
		});

		// Render google auth button
		google.accounts.id.renderButton(googleButton, {
			type: "standard",
			text: "signin",
			size: "large",
			width: "400",
			theme: "filled_black"
		});
	}

	/**
	 *
	 * @param res {CredentialResponse} Google credentials response
	 */
	async function onGoogleIdentify(res: CredentialResponse) {
		const token: string = res.credential;
		console.debug("Google Authenticated", token);

		googleToken = token;

		dialogNext.showModal();
		// TODO: Check authentication with backend
	}

	let dialogNext: HTMLDialogElement;
</script>

<!-- Google Client script for authentication -->
<svelte:head>
	<script
		src="https://accounts.google.com/gsi/client"
		async
		defer
		nonce="google-client"
		on:load={onScriptLoaded}
	></script>
</svelte:head>
