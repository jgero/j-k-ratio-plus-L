<script lang="ts">
    import { dev } from "$app/environment";
    import Editor from "./Editor.svelte";
    import Loading from "./Loading.svelte";
    import ErrorComponent from "./Error.svelte";

    const defaultSrc = [
        "fun hello() {",
        '\tprint("hello aphrodite")',
        "}",
    ].join("\n");

    let kotlinSrc = defaultSrc;
    let compilePromise: Promise<any> = Promise.reject({
        error: "nothing to compile yet",
    });

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
    <div class="content-wrapper">
        <h1>Kotlin</h1>
        <div class="editor-box">
            <Editor language="kotlin" bind:value={kotlinSrc} />
        </div>
    </div>
    <div class="button-box">
        <button on:click={onCompile}>compile</button>
    </div>
    <div class="content-wrapper">
        <h1>Java</h1>
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
                <ErrorComponent message={error} />
            {/await}
        </div>
    </div>
</div>

<style>
    :global(body, html) {
        margin: 0;
        height: 100%;
    }
    :global(*) {
        box-sizing: border-box;
    }
    .content {
        display: flex;
        flex-direction: row;
        background: #1e1e1e;
        height: 100%;
    }
    .content-wrapper {
        height: 100%;
        flex: 5;
        display: flex;
        flex-direction: column;
        align-items: center;
    }
    .editor-box {
        position: relative;
        width: 40vw;
        height: 80vh;
        display: flex;
        align-items: center;
        justify-content: center;
    }
    h1 {
        color: white;
        text-align: center;
    }
    .button-box {
        flex: 1;
    }
</style>
