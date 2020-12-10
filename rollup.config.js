import svelte from 'rollup-plugin-svelte';
import commonjs from '@rollup/plugin-commonjs';
import resolve from '@rollup/plugin-node-resolve';
import livereload from 'rollup-plugin-livereload';
import { terser } from 'rollup-plugin-terser';
import css from 'rollup-plugin-css-only';
import rust from "@wasm-tool/rollup-plugin-rust";

const production = !process.env.ROLLUP_WATCH;

function serve() {
	let server;

	function toExit() {
		if (server) server.kill(0);
	}

	return {
		writeBundle() {
			if (server) return;
			server = require('child_process').spawn('npm', ['run', 'start', '--', '--dev'], {
				stdio: ['ignore', 'inherit', 'inherit'],
				shell: true
			});

			process.on('SIGTERM', toExit);
			process.on('exit', toExit);
		}
	};
}

export default {
  input: "src/main",
	output: {
		sourcemap: true,
		format: 'iife',
		name: 'app',
		dir: 'public/build/',
	},
	plugins: [
    rust({
    // Whether to build in debug mode or release mode.
    // In watch mode this defaults to true.
    debug: false,

    // Whether to display extra compilation information in the console.
    verbose: false,

    // Directory on your server where the .wasm files will be loaded from.
    // This is prepended to the URL, so you should put a / at the end of the directory, e.g. "/foo/".
    serverPath: "",

    // Extra arguments to pass to `cargo build`.
    cargoArgs: [],

    // Whether the code will be run in Node.js or not.
    //
    // This is needed because Node.js does not support `fetch`.
    nodejs: false,

    // Whether to inline the `.wasm` file into the `.js` file.
    //
    // This is slower and it increases the file size by ~33%,
    // but it does not require a separate `.wasm` file.
    //
    // If this is `true` then `serverPath`, `nodejs`,
    // and `importHook` will be ignored.
    inlineWasm: false,

    // Which files it should watch in watch mode. This is relative to the crate directory.
    // Supports all of the glob syntax.
    watchPatterns: ["src/**"],

    // Allows you to customize the behavior for loading the .wasm file
    importHook: function (path) { return JSON.stringify(path); },

    }),
		svelte({
			compilerOptions: {
				// enable run-time checks when not in production
				dev: !production
			}
		}),
		// we'll extract any component CSS out into
		// a separate file - better for performance
		css({ output: 'bundle.css' }),

		// If you have external dependencies installed from
		// npm, you'll most likely need these plugins. In
		// some cases you'll need additional configuration -
		// consult the documentation for details:
		// https://github.com/rollup/plugins/tree/master/packages/commonjs
		resolve({
			browser: true,
			dedupe: ['svelte']
		}),
		commonjs(),

		// In dev mode, call `npm run start` once
		// the bundle has been generated
		!production && serve(),

		// Watch the `public` directory and refresh the
		// browser on changes when not in production
		!production && livereload('public'),

		// If we're building for production (npm run build
		// instead of npm run dev), minify
		production && terser()
	],
	watch: {
		clearScreen: false
	}
};
