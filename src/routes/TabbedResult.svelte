<script lang="ts">
    import Editor from "./Editor.svelte";

    export let javaResult: string[];
    let tabIndex = 0;
    let currentTabContent: string;
    $: {
        currentTabContent = javaResult[tabIndex];
    }
</script>

<section>
    <ol>
        {#each javaResult as _, i}
            <li
                on:click={() => (tabIndex = i)}
                class={tabIndex === i ? "selected" : ""}
            >
                {i + 1}
            </li>
        {/each}
    </ol>
    <Editor
        language="java"
        value={currentTabContent}
        reactive={true}
        readonly={true}
    />
</section>

<style>
    section {
        position: relative;
        width: 100%;
        height: 100%;
    }
    ol {
        position: absolute;
        top: -2rem;
        margin: 0;
        padding: 0;
        color: white;
        display: flex;
        flex-direction: row;
        width: 100%;
        list-style: none;
        padding-inline-start: 1rem;
    }
    li {
        padding-inline: 0.5rem;
        margin-inline: 0.5rem;
        cursor: pointer;
        font-weight: bold;
    }
    li.selected {
        text-shadow: 0 0 5px;
    }
</style>
