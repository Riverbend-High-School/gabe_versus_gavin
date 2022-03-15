<template>
    <div class="container">
        <div class="content">
            <h1>Create New Poll</h1>
            <table>
                <tbody>
                    <tr>
                        <td>
                            <p>Title</p>
                        </td>
                        <td>
                            <input
                                class="text-input"
                                type="text"
                                v-model="title"
                            />
                        </td>
                    </tr>
                    <tr>
                        <td><p>Option 1</p></td>
                        <td>
                            <input
                                class="text-input"
                                type="text"
                                v-model="options[0]"
                            />
                        </td>
                    </tr>
                    <tr>
                        <td><p>Option 2</p></td>
                        <td>
                            <input
                                class="text-input"
                                type="text"
                                v-model="options[1]"
                            />
                        </td>
                    </tr>
                    <tr>
                        <td><p>Option 3</p></td>
                        <td>
                            <input
                                class="text-input"
                                type="text"
                                v-model="options[2]"
                            />
                        </td>
                    </tr>
                    <tr>
                        <td><p>Option 4</p></td>
                        <td>
                            <input
                                class="text-input"
                                type="text"
                                v-model="options[3]"
                            />
                        </td>
                    </tr>
                    <tr>
                        <td><p>Active?</p></td>
                        <td>
                            <label class="switch">
                                <input type="checkbox" v-model="active" />
                                <span class="slider round"></span>
                            </label>
                        </td>
                    </tr>
                </tbody>
            </table>

            <input
                class="submit-button"
                type="submit"
                value="Submit"
                v-on:click="post"
            />
            <h1>Current Poll Control</h1>
            <div class="active-buttons">
                <input
                    type="button"
                    value="Activate Poll"
                    v-on:click="activate"
                />
                <input
                    type="button"
                    value="Deactivate Poll"
                    v-on:click="deactivate"
                />
            </div>
        </div>
    </div>
</template>

<script>
const BASE_URL = "";

export default {
    name: "admin-page",
    data() {
        return {
            title: "",
            options: ["", "", "", ""],
            active: true,
        };
    },
    methods: {
        post() {
            let final_options = this.options.filter((option) => option !== "");

            this.axios
                .post(`${BASE_URL}/api/verysecureadminpath`, {
                    title: this.title,
                    options: final_options,
                    active: this.active,
                })
                .then((response) => {
                    if (response.data.status != 200) {
                        alert(
                            `Failed to make new poll with error: ${response.data.message}`
                        );
                    } else {
                        alert("Successfully made new poll");
                    }
                    // console.log(`Got poll: ${JSON.stringify(response.data)}`);
                })
                .catch((error) => {
                    alert(
                        `Failed to make new poll with unknown error. Please check console for details.`
                    );
                    console.error(error);
                });
        },
        deactivate() {
            this.axios
                .post(`${BASE_URL}/api/verysecureadminpath/deactivate`)
                .then((response) => {
                    if (response.data.status != 200) {
                        alert(
                            `Failed to make deactivate poll with error: ${response.data.message}`
                        );
                    } else {
                        alert("Successfully deactivated poll!");
                    }
                    // console.log(`Got poll: ${JSON.stringify(response.data)}`);
                })
                .catch((error) => {
                    alert(
                        `Failed to make deactivate poll with unknown error. Please check console for details.`
                    );
                    console.error(error);
                });
        },
        activate() {
            this.axios
                .post(`${BASE_URL}/api/verysecureadminpath/activate`)
                .then((response) => {
                    if (response.data.status != 200) {
                        alert(
                            `Failed to make activate poll with error: ${response.data.message}`
                        );
                    } else {
                        alert("Successfully activate poll!");
                    }
                    // console.log(`Got poll: ${JSON.stringify(response.data)}`);
                })
                .catch((error) => {
                    alert(
                        `Failed to make activate poll with unknown error. Please check console for details.`
                    );
                    console.error(error);
                });
        },
    },
};
</script>

<style>
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

.text-input {
    width: 100%;
    height: 40px;
    border: 1px solid #333;
    border-radius: 5px;
    padding: 5px;
    margin: 5px;
}

h1,
p {
    color: white;
}

/* The switch - the box around the slider */
.switch {
    position: relative;
    display: inline-block;
    width: 60px;
    height: 34px;
}

/* Hide default HTML checkbox */
.switch input {
    opacity: 0;
    width: 0;
    height: 0;
}

/* The slider */
.slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: #ccc;
    -webkit-transition: 0.4s;
    transition: 0.4s;
}

.slider:before {
    position: absolute;
    content: "";
    height: 26px;
    width: 26px;
    left: 4px;
    bottom: 4px;
    background-color: white;
    -webkit-transition: 0.4s;
    transition: 0.4s;
}

input:checked + .slider {
    background-color: #2196f3;
}

input:focus + .slider {
    box-shadow: 0 0 1px #2196f3;
}

input:checked + .slider:before {
    -webkit-transform: translateX(26px);
    -ms-transform: translateX(26px);
    transform: translateX(26px);
}

/* Rounded sliders */
.slider.round {
    border-radius: 34px;
}

.slider.round:before {
    border-radius: 50%;
}

.active-buttons {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    width: 100%;
}
.active-buttons > input {
    margin: 5px;
}

.active-buttons > input,
.submit-button {
    display: inline-block;
    padding: 0.3em 1.2em;
    margin: 0 0.1em 0.1em 0;
    border: 0.16em solid rgba(255, 255, 255, 0);
    border-radius: 2em;
    box-sizing: border-box;
    text-decoration: none;
    font-family: "Roboto", sans-serif;
    font-weight: 300;
    color: black;
    text-shadow: 0 0.04em 0.04em rgba(0, 0, 0, 0.35);
    text-align: center;
    transition: all 0.2s;
}
.active-buttons > input:hover,
.submit-button:hover {
    border-color: rgba(255, 255, 255, 1);
}
@media all and (max-width: 30em) {
    .active-buttons > input,
    .submit-button {
        display: block;
        margin: 0.2em auto;
    }
}
</style>
