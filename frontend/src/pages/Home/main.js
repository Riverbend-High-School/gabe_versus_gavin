import { createApp } from "vue";
import Home from "./Home.vue";

import axios from "axios";
import VueAxios from "vue-axios";

const app = createApp(Home);
app.use(VueAxios, axios);
app.mount("#app");
