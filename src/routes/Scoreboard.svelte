<script lang="ts">
    import { dev } from "$app/environment";
    import { onDestroy, onMount } from "svelte";
    const apiUrl = dev ? "http://localhost:8080/scoreboard" : "scoreboard";
    let scoreboard: any[] = [];
    let intervalHandle: any;

    onMount(() => {
        intervalHandle = setInterval(refresh, 5000);
    });

    onDestroy(() => {
        if (intervalHandle) {
            clearInterval(intervalHandle);
        }
    });

    async function refresh() {
        let response = await self.fetch(apiUrl);

        if (response.ok) {
            const resJson = await response.json();
            if (resJson.error) {
                throw new Error(resJson.error);
            }
            scoreboard = resJson;
        } else {
            throw new Error("received error from API");
        }
    }
</script>

<aside>
    <ol>
        {#each scoreboard as entry}
            <li>
                <span>{entry.user}</span>
                <span>{entry.ratio.chars}</span>
                <span>{entry.ratio.lines}</span>
            </li>
        {/each}
    </ol>
    <span><b>SCOREBOARD</b></span>
</aside>

<style>
    aside {
        font-family: "JetBrains Mono";
        width: 30vw;
        position: absolute;
        z-index: 100;
        top: 0;
        left: 35vw;
        display: flex;
        flex-direction: column;
        align-items: center;
        background: #1e1e1e;
        color: white;
        border: 1px solid white;
        border-top: transparent;
        padding-block: 1rem;
        border-bottom-left-radius: 0.5rem;
        border-bottom-right-radius: 0.5rem;
        transform: translateY(calc(-100% + 3rem));
        transition: transform ease-out 0.3s;
    }
    aside:hover {
        transform: translateY(0);
    }
    aside > span {
        margin-block-start: 1rem;
    }
    ol {
        padding: 0;
        margin: 0;
    }
</style>
