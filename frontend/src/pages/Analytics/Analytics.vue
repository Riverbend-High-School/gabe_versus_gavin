<template>
    <div v-if="poll" class="container">
        <div class="content">
            <div v-if="poll.title == ''">
                <h1>No Current Poll</h1>
            </div>
            <div v-else class="chart-top">
                <h1 class="title">{{ poll.title }}</h1>
                <div class="chart-wrap">
                    <div class="chart-container">
                        <apexchart
                            ref="chart"
                            type="pie"
                            :options="chartOptions"
                            :series="poll.votes"
                            height="100%"
                        ></apexchart>
                    </div>
                </div>
            </div>
        </div>
    </div>
    <div v-else class="loading-container">
        <p class="loading">Loading...</p>
    </div>
</template>

<script>
// import { useCookies } from "vue3-cookies";
import VueApexCharts from "vue3-apexcharts";

const isEqual = (...objects) =>
    objects.every((obj) => JSON.stringify(obj) === JSON.stringify(objects[0]));
const BASE_URL = "";

export default {
    name: "analytics-page",
    // setup() {
    //     const { cookies } = useCookies();
    //     return { cookies };
    // },
    components: {
        apexchart: VueApexCharts,
    },
    data() {
        return {
            poll: null,
            interval: null,
            chartOptions: {
                chart: {
                    animation: {
                        easing: "easein",
                    },
                    fontFamily: "Patrick Hand, cursive",
                },
                dataLabels: {
                    enabled: true,
                    style: {
                        fontSize: "2rem",
                    },
                },
                stroke: {
                    show: false,
                },
                legend: {
                    position: "top",
                    offsetY: 0,
                    fontSize: "35rem",
                    labels: {
                        colors: ["#FFFFFF"],
                    },
                    markers: {
                        width: 20,
                        height: 20,
                    },
                    itemMargin: {
                        horizontal: 10,
                        vertical: 0,
                    },
                },
                colors: ["#d86c70e3", "#75c4aee3", "#d3d2b5e3", "#6cdff9e3"],
                labels: ["", "", "", ""],
            },
            current_labels: ["", "", "", ""],
        };
    },
    methods: {
        get_poll() {
            // console.log(this.cookies.get("id"));
            this.axios
                .get(`${BASE_URL}/api/results`, {
                    // withCredentials: true,
                })
                .then((response) => {
                    if (
                        this.$refs.chart &&
                        this.poll &&
                        this.poll.options != this.current_labels
                    ) {
                        // console.log(`Updating chart ${this.current_labels}`);
                        this.$refs.chart.updateOptions({
                            labels: this.poll.options,
                        });
                        this.current_labels = this.poll.options;
                    }
                    if (response.data.status == 200) {
                        if (!isEqual(this.poll, response.data.data)) {
                            // console.log("Updating poll");
                            this.poll = response.data.data;
                        }
                    }
                    // console.log(`Got poll: ${JSON.stringify(response.data)}`);
                })
                .catch((error) => {
                    console.error(error);
                });
        },
    },
    mounted: function () {
        this.interval = setInterval(() => {
            this.get_poll();
        }, 800);
    },
    unmounted() {
        clearInterval(this.interval);
    },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.container {
    text-align: center;
    justify-items: center;
    max-height: 100vh;
    overflow: hidden;
    background-color: #0f172a;
    height: 100vh;
}
.content {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    height: 100vh;
}
.chart-container {
    width: 100%;
    min-height: 60vh;
    height: 60vh;
    font-size: 0.25em;
    margin: 0;
}
.chart-top {
    width: 100vw;
}
.title {
    color: white;
    font-size: 5rem;
    margin: 1px;
}

.loading-container {
    max-height: 100vh;
    height: 100vh;
    overflow: hidden;
    background-color: #0f172a;
}
.loading {
    color: white;
}
</style>
