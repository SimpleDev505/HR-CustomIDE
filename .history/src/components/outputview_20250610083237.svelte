<script lang="ts">
  import {
    EDITOR_AREA,
    OPEN_TABS_SELECTED_FILEPATH,
    OUTPUTS,
  } from "$lib/store";
  import axios from "axios";
  function GetLanguageExtension() {
    let path = $OPEN_TABS_SELECTED_FILEPATH
      ? $OPEN_TABS_SELECTED_FILEPATH.toString()
      : "";
    if (path) {
      const fileExtension = path.split(".").pop()?.toLowerCase();
      if (fileExtension) {
        switch (fileExtension) {
          case "js":
          case "ts":
            return "javascript"; // 'javascript' is the ID for JS/TS
          case "py":
            return "python";
          case "html":
          case "htm":
            return "html";
          case "css":
            return "css";
          case "json":
            return "json";
          case "xml":
            return "xml";
          case "md":
            return "markdown";
          // Add more cases for other languages you want to support.
          // You can find Monaco's built-in language IDs in their documentation.
          default:
            return "plaintext"; // Default to plain text for unrecognized extensions.
        }
      }
    }
  }
  function GetLanguageId(lang: string) {
    switch (lang) {
      case "javascript":
        return 93;
      case "python":
        return 95;
      default:
        return null;
    }
  }
  export async function FetchOuput() {
    if (!$EDITOR_AREA || $OPEN_TABS_SELECTED_FILEPATH === "") {
      return;
    }
    const _editor_content = $EDITOR_AREA.getValue();
    const fileName =
      $OPEN_TABS_SELECTED_FILEPATH.split("/").pop() || "untitled";
    const Language = GetLanguageExtension();
    const LanguageId = GetLanguageId(Language ?? "");
    const timestamp = new Date().toLocaleString();
    if (LanguageId === null) {
      const statusMessage = `Compilation failed: Language '${Language}' not supported for compilation.`;
      compiledOutputs.update((outputs) => [
        ...outputs,
        {
          timestamp,
          fileName,
          status: "error",
          output: "N/A",
          error: [
            `Language '${monacoLanguageId}' cannot be compiled by Judge0.`,
          ], // Corrected: Wrapped in array
        },
      ]);
      setTimeout(() => {
        statusMessage = "";
      }, 5000);
      return;
    }
    const options = {
      method: "POST",
      url: "https://judge0-ce.p.rapidapi.com/submissions",
      params: {
        base64_encoded: "true",
        wait: "false",
        fields: "*",
      },
      headers: {
        "x-rapidapi-key": "32088943a9msh0a19c2a436be580p1031bfjsnff9fa09908b3",
        "x-rapidapi-host": "judge0-ce.p.rapidapi.com",
        "Content-Type": "application/json",
      },
      data: {
        language_id: 52,
        source_code:
          "I2luY2x1ZGUgPHN0ZGlvLmg+CgppbnQgbWFpbih2b2lkKSB7CiAgY2hhciBuYW1lWzEwXTsKICBzY2FuZigiJXMiLCBuYW1lKTsKICBwcmludGYoImhlbGxvLCAlc1xuIiwgbmFtZSk7CiAgcmV0dXJuIDA7Cn0=",
        stdin: "SnVkZ2Uw",
      },
    };

    try {
      const response = await axios.request(options);
      console.log(response.data);
    } catch (error) {
      console.error(error);
    }
  }
</script>

<div class="out-cnt">
  {#each $OUTPUTS as output}{/each}
</div>

<style>
  .out-cnt {
    display: flex;
    flex-direction: column;
    row-gap: 2px;
    width: 100%;
    height: 100%;
    z-index: -1;
  }
</style>
