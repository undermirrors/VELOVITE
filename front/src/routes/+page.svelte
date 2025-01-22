<script lang="ts">
	let name = 'Svelte';
	let src = "./Wink_parodie_lego_movie.png";;
	
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>

<h1>Hello world!</h1>
<h1>Hello {name.toUpperCase()}!</h1>

<img {src} alt="{name} winks." />
<style>
	p {
		color: goldenrod;
		font-family: 'Comic Sans MS', cursive;
		font-size: 2em;
	}
</style>