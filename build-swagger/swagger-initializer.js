window.onload = function () {

    //<editor-fold desc="Changeable Configuration Block">
    window.ui = SwaggerUIBundle({
        urls: [
            { url: "./specs/spec-crowdproj-ad-root.yaml", name: "Root methods" },
            { url: "./specs/spec-crowdproj-ad-v1.yaml", name: "API v1" }
        ],
        "dom_id": "#swagger-ui",
        deepLinking: true,
        presets: [
            SwaggerUIBundle.presets.apis,
            SwaggerUIStandalonePreset
        ],
        plugins: [
            SwaggerUIBundle.plugins.DownloadUrl
        ],
        layout: "StandaloneLayout",
        queryConfigEnabled: false,
    })

    //</editor-fold>

};
