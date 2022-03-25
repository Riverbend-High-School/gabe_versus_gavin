<template>
    <div>
        <div class="container">
            <div v-if="poll == 'success'">
                <div class="success">
                    <h1>Thanks for voting!</h1>
                    <p>
                        Your vote has been recorded. The results will be
                        available soon!
                    </p>
                </div>
            </div>
            <div v-else-if="poll">
                <div v-if="poll.title == '' || !poll.active" class="success">
                    <h1>No Active Poll</h1>
                    <p>Please check back soon!</p>
                </div>
                <div v-else :style="cssProps">
                    <div class="options">
                        <div
                            v-for="(option, index) in poll.options"
                            v-bind:key="index"
                            :class="'option option' + (index + 1)"
                            v-on:click="vote(index)"
                        >
                            <p>{{ option }}</p>
                            <!-- <p>{{ poll.votes[index] }}</p> -->
                        </div>
                    </div>
                </div>
            </div>
            <div v-else class="success">
                <h1>Loading...</h1>
            </div>
        </div>
        <!-- <div class="container">Pie Chart</div> -->
    </div>
</template>

<script>
// import { useCookies } from "vue3-cookies";

const BASE_URL = "";

export default {
    name: "home-page",
    // setup() {
    //     const { cookies } = useCookies();
    //     return { cookies };
    // },
    data() {
        return {
            poll: null,
            interval: null,
        };
    },
    methods: {
        get_poll() {
            // console.log(this.cookies.get("id"));
            this.axios
                .get(`${BASE_URL}/api/results`, {
                    withCredentials: true,
                })
                .then((response) => {
                    if (response.data.status == 200) {
                        this.poll = response.data.data;
                    }
                    // console.log(`Got poll: ${JSON.stringify(response.data)}`);
                })
                .catch((error) => {
                    console.error(error);
                });
        },
        vote(option) {
            // console.log("Voting for option: " + option);
            // console.log(this.cookies.get("id"));
            this.axios
                .post(`${BASE_URL}/api/vote/${option}`, {
                    withCredentials: true,
                })
                .then((response) => {
                    if (response.data.status != 200) {
                        alert(
                            `Failed to vote with error: ${response.data.message}`
                        );
                    } else {
                        if (
                            response.data.message != "Successfully undid vote!"
                        ) {
                            this.poll = "success";
                            clearInterval(this.interval);
                        }
                    }
                    // console.log(`Got poll: ${JSON.stringify(response.data)}`);
                })
                .catch((error) => {
                    console.error(error);
                });
        },
    },
    computed: {
        cssProps() {
            if (this.poll != "success") {
                return {
                    "--num-options": this.poll.options.length,
                };
            } else {
                return {
                    "--num-options": 1,
                };
            }
        },
    },
    mounted: function () {
        this.axios
            .post(`${BASE_URL}/api/vote/check_vote_status/`, {
                withCredentials: true,
            })
            .then((response) => {
                if (response.data.status != 200) {
                    this.poll = "success";
                } else {
                    this.interval = setInterval(() => {
                        this.get_poll();
                    }, 800);
                }
            })
            .catch((error) => {
                this.poll = "success";
                console.error(error);
            });
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
    height: 100vh;
    background-color: #0f172a;
    overflow: hidden;
}

.options {
    height: 100vh;
    width: 100vw;
    display: flex;
    flex-direction: column;
    font-size: calc(4em / var(--num-options));
}
.options > .option {
    display: flex;
    flex-direction: column;
    height: calc(100vh / var(--num-options));
    justify-content: center;
    align-items: center;
    margin: 10px 10px 0px 10px;

    border-radius: 10px;
}
.option:last-child {
    margin-bottom: 10px;
}
.option > p {
    font-size: 4em;
    margin: 0;
}
.options > .option1 {
    background-color: #d86c70e3;
}
.options > .option1:hover {
    background-color: #d86c708c;
}
.options > .option2 {
    background-color: #75c4aee3;
}
.options > .option2:hover {
    background-color: #75c4ae8c;
}
.options > .option3 {
    background-color: #d3d2b5e3;
}
.options > .option3:hover {
    background-color: #d3d2b58c;
}
.options > .option4 {
    background-color: #6cdff9e3;
}
.options > .option4:hover {
    background-color: #6cdff98c;
}
.success {
    display: flex;
    flex-direction: column;
    height: 100vh;
    width: 100vw;
    justify-content: center;
    align-items: center;
    background-color: #0f172a;
    color: white;
}
.success > p {
    font-size: 2em;
    margin: 0;
}
.success > h1 {
    font-size: 4rem;
    margin: 0;
}
</style>
