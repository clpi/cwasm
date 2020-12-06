<script lang="ts">
	export let name: string;
</script>

<main>
  <!-- <img src="build/img/default.png"/> -->
  <img  src="build/img/profile.png"/>
  <p>Visit <a href="http://chris.pecunies.com">chris.pecunies.com</a> for now!</p>
  <p>Email me at clp @t clp d0t is for any questions!</p>
</main>

<style>
	main {
		text-align: center;
    overflow: hidden;
    color: #fdf3fb;
    background: #261b24;
      margin: 0;
      padding: 0;
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
