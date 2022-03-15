import { createApp } from "vue";
import Analytics from "./Analytics.vue";

import axios from "axios";
import VueAxios from "vue-axios";

const app = createApp(Analytics);
app.use(VueAxios, axios);
app.mount("#app");
