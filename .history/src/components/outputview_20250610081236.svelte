<script>
  import { OUTPUTS } from "$lib/store";
  import axios from "axios";
  const RAPIDAPI_HOST = "judge0-ce.p.rapidapi.com";
  const RAPIDAPI_KEY = import.meta.env?.VITE_RAPIDAPI_KEY || "";
  const JUDGE0_SUBMISSION_URL = `https://${RAPIDAPI_HOST}/submissions?base64_encoded=false&wait=true`;
  async function FetchOuput() {
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
