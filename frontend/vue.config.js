module.exports = {
    pages: {
        index: {
            entry: "./src/pages/Home/main.js",
            template: "public/index.html",
            title: "Gabe VS Gavin",
            chunks: ["chunk-vendors", "chunk-common", "index"],
        },
        graphic: {
            entry: "./src/pages/Analytics/main.js",
            template: "public/index.html",
            title: "GvG Graphic",
            chunks: ["chunk-vendors", "chunk-common", "graphic"],
        },
        verysecureadminpath: {
            entry: "./src/pages/Admin/main.js",
            template: "public/index.html",
            title: "Gabe VS Gavin Admin",
            chunks: ["chunk-vendors", "chunk-common", "verysecureadminpath"],
        },
    },
};
