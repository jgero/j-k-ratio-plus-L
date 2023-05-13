<script lang="ts">
    import { dev } from "$app/environment";
    import Editor from "./Editor.svelte";
    import Loading from "./Loading.svelte";
    import ErrorComponent from "./Error.svelte";
    import IconBanner from "./IconBanner.svelte";
    import Button from "./Button.svelte";
    import TabbedResult from "./TabbedResult.svelte";

    const defaultSrc = [
        "fun hello() {",
        '\tprint("hello aphrodite")',
        "}",
    ].join("\n");

    let kotlinSrc = defaultSrc;
    let compilePromise: Promise<string[]> = Promise.reject(
        "nothing to compile yet"
    );

    const apiUrl = dev ? "http://localhost:8080/compile" : "compile";

    function onCompile() {
        compilePromise = compile();
    }

    async function compile() {
        let response = await self.fetch(apiUrl, {
            method: "POST",
            headers: new Headers([["Content-Type", "application/json"]]),
            body: JSON.stringify({ src: kotlinSrc }),
        });

        if (response.ok) {
            const resJson = await response.json();
            if (resJson.error) {
                throw new Error(resJson.error);
            }
            return resJson.src;
        } else {
            throw new Error("received error from API");
        }
    }
</script>

<div class="content">
    <h1>Kotlin</h1>
    <div />
    <h1>Java</h1>
    <div class="editor-box">
        <Editor language="kotlin" bind:value={kotlinSrc} />
    </div>
    <Button on:click={onCompile} />
    <div class="editor-box">
        {#await compilePromise}
            <Loading />
        {:then javaResult}
            <TabbedResult {javaResult} />
        {:catch error}
            <ErrorComponent message={error} />
        {/await}
    </div>
</div>
<IconBanner />

<style>
    :global(body, html) {
        margin: 0;
        height: 100%;
    }
    :global(*) {
        box-sizing: border-box;
    }
    :global(:root) {
        @font-face {
            font-family: JetBrains Mono;
            src: url("JetBrainsMono-Regular.ttf");
        }
    }
    .content {
        display: grid;
        grid-template-columns: 4fr 1fr 4fr;
        grid-template-rows: 1fr 6fr;
        background: #1e1e1e;
        height: 100%;
    }
    .editor-box {
        position: relative;
        display: flex;
        align-items: center;
        justify-content: center;
    }
    h1 {
        color: white;
        font-family: "JetBrains Mono";
        margin: auto;
        font-size: 3rem;
    }
    h1:first-of-type {
        text-shadow: 0 0 20px lime;
    }
    h1:nth-of-type(2) {
        text-shadow: 0 0 20px red;
    }
</style>
