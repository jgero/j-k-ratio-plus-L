<script lang="ts">
    import { dev } from "$app/environment";
    import { onDestroy, onMount } from "svelte";
    export let username: string;
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
            scoreboard = (resJson as any[]).slice(0, 10);
        } else {
            throw new Error("received error from API");
        }
    }
</script>

<aside>
    <table>
        <thead>
            <tr>
                <td>RANK</td>
                <td>ALIAS</td>
                <td>CHAR RATIO</td>
                <td>LINE RATIO</td>
            </tr>
        </thead>
        <tbody>
            {#each scoreboard as entry, index}
                <tr>
                    <td>{index + 1}</td>
                    <td>{entry.user}</td>
                    <td>{entry.ratio.chars}</td>
                    <td>{entry.ratio.lines}</td>
                </tr>
            {/each}
        </tbody>
    </table>
    <span>{username}</span>
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
        margin-block-start: 0.7rem;
    }
    table + span {
        opacity: 0.5;
    }
    table {
        width: 100%;
    }
    td {
        text-align: center;
    }
</style>
