<script lang="ts">
    import type monaco from "monaco-editor";
    import { onMount } from "svelte";
    import editorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker";

    let divEl: HTMLDivElement;
    let editor: monaco.editor.IStandaloneCodeEditor;
    let Monaco: typeof monaco;

    export let language = "kotlin";
    export let reactive = false;
    export let readonly = false;
    export let value: string = [
        "fun main() {",
        '\tprint("Hello world!")',
        "}",
    ].join("\n");
    $: {
        if (editor && reactive) {
            editor.setValue(value);
        }
    }

    onMount(async () => {
        // @ts-ignore
        self.MonacoEnvironment = {
            getWorker: function (_moduleId: any, _label: string) {
                return new editorWorker();
            },
        };

        Monaco = await import("monaco-editor");
        editor = Monaco.editor.create(divEl, {
            value: value,
            language: language,
            theme: "vs-dark",
            readOnly: readonly
        });
        editor.onDidChangeModelContent((_) => {
            value = editor.getValue();
        });
        return () => {
            editor.dispose();
        };
    });
</script>

<div bind:this={divEl} class="monaco" />

<style>
    .monaco {
        height: 80vh;
    }
</style>
