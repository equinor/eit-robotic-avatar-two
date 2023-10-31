export default class Viewport {
    constructor(canvas) {
        this.canvas = canvas;
        this.gl = canvas.getContext("webgl");

        this.gl.clearColor(0.0, 0.0, 0.0, 1.0);
        this.gl.clear(this.gl.COLOR_BUFFER_BIT);

        const program = initShaderProgram(this.gl);

        console.log("Viewport.constructor", canvas)
    }

    update(bitmap) {
        console.log("Viewport.update", bitmap)
    }
}

function initShaderProgram(gl) {
    const vertexShader = loadShader(gl, gl.VERTEX_SHADER, `
        attribute vec2 aVertexPosition;
        attribute vec2 aTextureCoord;
    
        varying highp vec2 vTextureCoord;
    
        void main(void) {
            gl_Position = vec4(aVertexPosition, 0.0, 1.0);
            vTextureCoord = aTextureCoord;
        }
    `);
    const fragmentShader = loadShader(gl, gl.FRAGMENT_SHADER, `
        varying highp vec2 vTextureCoord;
        uniform sampler2D uSampler;

        void main(void) {
            gl_FragColor = texture2D(uSampler, vTextureCoord);
        }
    `);

    const shaderProgram = gl.createProgram();
    gl.attachShader(shaderProgram, vertexShader);
    gl.attachShader(shaderProgram, fragmentShader);
    gl.linkProgram(shaderProgram);

    if (!gl.getProgramParameter(shaderProgram, gl.LINK_STATUS)) {
        console.error("Unable to initialize the shader program:", gl.getProgramInfoLog(shaderProgram));
        return {};
    }

    return {
        program: shaderProgram,
        attribLocations: {
            vertexPosition: gl.getAttribLocation(shaderProgram, "aVertexPosition"),
            textureCoord: gl.getAttribLocation(shaderProgram, "aTextureCoord"),
        },
        uniformLocations: {
            uSampler: gl.getUniformLocation(shaderProgram, "uSampler"),
        },
    };
}

function loadShader(gl, type, source) {
    const shader = gl.createShader(type);
    gl.shaderSource(shader, source);
    gl.compileShader(shader);
    if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
        console.error("An error occurred compiling the shaders:", gl.getShaderInfoLog(shader));
        gl.deleteShader(shader);
        return null;
    }

    return shader;
}