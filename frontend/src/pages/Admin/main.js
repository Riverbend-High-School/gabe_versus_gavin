import { createApp } from "vue";
import Admin from "./Admin.vue";

import axios from "axios";
import VueAxios from "vue-axios";

const app = createApp(Admin);
app.use(VueAxios, axios);
app.mount("#app");
