<script lang="ts">
    import { dev } from "$app/environment";
    import Editor from "./Editor.svelte";
    import Loading from "./Loading.svelte";
    import Error from "./Error.svelte";

    const defaultSrc = ["fun hello() {", '\tprint("hello world")', "}"].join(
        "\n"
    );

    let kotlinSrc = defaultSrc;
    let compilePromise: Promise<any> = Promise.reject({ error: "nothing to compile yet" });

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
            return resJson.src;
        }
    }
</script>

<div class="content">
    <div class="editor-box">
        <Editor language="kotlin" bind:value={kotlinSrc} />
    </div>
    <div class="button-box">
        <button on:click={onCompile}>compile</button>
    </div>
    <div class="editor-box">
      {#await compilePromise}
        <Loading />
      {:then javaResult}
        <Editor
            language="java"
            value={javaResult}
            reactive={true}
            readonly={true}
        />
      {:catch error}
        <Error message={error.error} />
      {/await}
    </div>
</div>

<style>
    :global(body, html) {
        margin: 0;
        height: 100%;
    }
    .content {
        display: flex;
        flex-direction: row;
        background: black;
        height: 100%;
    }
    .editor-box {
        height: 100%;
        flex: 5;
        position: relative;
    }
    .button-box {
        flex: 1;
    }
</style>
