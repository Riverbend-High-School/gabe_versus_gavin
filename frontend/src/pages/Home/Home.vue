<template>
    <div>
        <div class="container">
            <div v-if="poll">
                <div v-if="poll.title == ''">
                    <h1>No Current Poll</h1>
                </div>
                <div v-else>
                    <h1 v-bind:modal="poll.title" class="title"></h1>
                    <div class="options">
                        <div
                            v-for="(option, index) in poll.options"
                            v-bind:key="index"
                            :class="'option option' + (index + 1)"
                            v-on:click="vote(index)"
                        >
                            <p>{{ option }}</p>
                            <p>{{ poll.votes[index] }}</p>
                        </div>
                    </div>
                </div>
            </div>
            <div v-else>
                <h1>Loading...</h1>
            </div>
        </div>
        <div class="container">Pie Chart</div>
    </div>
</template>

<script>
import { useCookies } from "vue3-cookies";

export default {
    name: "home-page",
    setup() {
        const { cookies } = useCookies();
        return { cookies };
    },
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
                .get("/api/results", {
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
                .post(`/api/vote/${option}`, {
                    withCredentials: true,
                })
                .then((response) => {
                    if (response.data.status != 200) {
                        alert(
                            `Failed to vote with error: ${response.data.message}`
                        );
                    } else {
                        this.get_poll();
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
    font-family: "Comic Sans MS", "Comic Sans", cursive;
}

.title {
    position: absolute;
    top: 0;
    left: 0;
}

.options {
    height: 100vh;
    width: 100vw;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    align-content: stretch;
    background-color: black;
    font-size: 5em;
}
.options > .option {
    display: flex;
    flex-direction: column;
    justify-content: center;
    color: white;
    margin: 0 0 0 0;
    height: 100%;
}
.option > p {
    margin: 0 0 0 0;
}
.options > .option1 {
    background-color: #d86c70e3;
}
.options > .option1:hover {
    background-color: #d86c708c;
}
.options > .option2 {
    background-color: #6cdff9e3;
}
.options > .option2:hover {
    background-color: #6cdff98c;
}
.options > .option3 {
    background-color: #75c4aee3;
}
.options > .option3:hover {
    background-color: #75c4ae8c;
}
.options > .option4 {
    background-color: #d3d2b5e3;
}
.options > .option4:hover {
    background-color: #d3d2b58c;
}
</style>
