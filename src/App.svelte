<script lang="ts">
	export let name: string;
</script>

<main>
  <h1>Chris Pecunies</h1>
  <p>Visit <a href="http://chris.pecunies.com">chris.pecunies.com</a> for now!</p>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
