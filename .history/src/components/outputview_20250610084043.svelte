<script lang="ts">
  import {
    EDITOR_AREA,
    JUDGE0_SUBMISSION_URL,
    OPEN_TABS_SELECTED_FILEPATH,
    OUTPUTS,
    RAPIDAPI_HOST,
    RAPIDAPI_KEY,
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
  async function FetchOuput() {
    if (!$EDITOR_AREA || $OPEN_TABS_SELECTED_FILEPATH === "") {
      return;
    }
    const _editor_content = $EDITOR_AREA.getValue();
    const fileName =
      $OPEN_TABS_SELECTED_FILEPATH.split("/").pop() || "untitled";
    const Language = GetLanguageExtension();
    const LanguageId = GetLanguageId(Language ?? "");
    const timestamp = new Date().toLocaleString();
    let statusMessage;
    if (LanguageId === null) {
      statusMessage = `Compilation failed: Language '${Language}' not supported for compilation.`;
      OUTPUTS.update((outputs) => [
        ...outputs,
        {
          timestamp,
          fileName,
          status: "error",
          output: "N/A",
          error: [`Language '${LanguageId}' cannot be compiled by Judge0.`], // Corrected: Wrapped in array
        },
      ]);
      setTimeout(() => {
        statusMessage = "";
      }, 5000);
      return;
    }
    const initialPendingEntry = {
      timestamp,
      fileName,
      status: "pending",
      output: "Submitting to Judge0...",
      error: undefined,
      judge0Status: "In Queue",
    };
    OUTPUTS.update((outputs) => [...outputs, initialPendingEntry]);
    const currentEntryIndex = $OUTPUTS.length - 1;
    try {
      const response = await axios.post(
        JUDGE0_SUBMISSION_URL,
        {
          language_id: LanguageId,
          source_code: _editor_content,
          cpu_time_limit: 2.0,
          memory_limit: 128000,
        },
        {
          headers: {
            "Content-Type": "application/json",
            "X-RapidAPI-Host": RAPIDAPI_HOST,
            "X-RapidAPI-Key": RAPIDAPI_KEY,
          },
        }
      );

      const result = response.data;

      let finalStatus;
      let output = result.stdout || "";
      let errorMsgs = [];
      let judge0Description = result.status?.description || "Unknown Status";

      if (result.stderr) errorMsgs.push(result.stderr);
      if (result.compile_output) errorMsgs.push(result.compile_output);
      if (result.message) errorMsgs.push(result.message);

      if (result.status?.id === 3) {
        finalStatus = "success";
      } else if (result.status?.id === 1 || result.status?.id === 2) {
        finalStatus = "pending";
        output =
          output || "Still processing, try again or check Judge0 status.";
      } else {
        finalStatus = "error";
        if (errorMsgs.length === 0) {
          errorMsgs.push(`Execution Status: ${judge0Description}`);
        }
      }

      OUTPUTS.update((outputs) => {
        outputs[currentEntryIndex] = {
          ...outputs[currentEntryIndex],
          status: finalStatus,
          output: output,
          error: errorMsgs.length > 0 ? errorMsgs : undefined,
          judge0Status: judge0Description,
        };
        return outputs;
      });
      statusMessage = `Compilation/Execution of '${fileName}' complete: ${judge0Description}.`;
    } catch (error) {
      console.error("Judge0 Compile Error (Axios):", error);
      let errorMessage = `Judge0 API failed`;

      if (axios.isAxiosError(error) && error.response) {
        errorMessage += ` - ${error.response.status}: ${JSON.stringify(error.response.data)}`;
      }

      OUTPUTS.update((outputs) => {
        outputs[currentEntryIndex] = {
          ...outputs[currentEntryIndex],
          status: "error",
          output: "API Error or Execution Problem.",
          error: [errorMessage],
          judge0Status: "API Error",
        };
        return outputs;
      });
      statusMessage = `Compilation of '${fileName}' failed due to API error.`;
    } finally {
      setTimeout(() => {
        statusMessage = "";
      }, 3000);
    }
  }
</script>

<div class="out-cnt">
  <button
    onclick={() => {
      FetchOuput();
      console.log($OUTPUTS);
    }}
  ></button>
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
