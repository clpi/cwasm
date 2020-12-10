<script>
	export let name;
    import { onMount } from 'svelte'
    //import  echo  from "cdom"
    //import * as cd from "cdom"
    import { echoTwo } from "cdom"

    let c;
     onMount(async () => {
      console.log(echoTwo("FDS"))
    });

</script>

<main>
  <h1>clp.is</h1>
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
