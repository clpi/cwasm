<script>
	export let name;
	let contactName = "";

  const contact = () => {
    if (contactName != "") {
        open("mailto://chris@pecu.io");
    }
  }
</script>

<main>
  <h1>{ name }</h1>
  <h2>welcome to clp.is</h2>
  <p>I'm Chris Pecunies. This website is under construction -- visit <a href="http://chris.pecunies.com">my other website</a> instead</p>
  <form name="contact">
    <span class="contactPrompt">Your name? </span>
    <input name="contactName" bind:value={contactName}/>
    <button on:click={contact}>contact me</button>
    </form>
  <hr/>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}
  .contactPrompt {
    color: #ff6e44;
    text-transform: lowercase;
    font-weight: 400;
    padding-right: 20px;
  }

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}
  h2 {
    color: #ff6e44;
    text-transform: lowercase;
    font-size: 2.2em;
    font-weight: 300;
  }

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
