export class GlContext {
    constructor(canvas) {
        this.gl = canvas.getContext('webgl');
    }
}

var gl_context;
export function set_context(context) {
    gl_context = context.gl;
}

export function enable(capability) {
    gl_context.enable(capability);
}
export function blend_func(sfactor, dfactor) {
    gl_context.blendFunc(sfactor, dfactor);
}
export function draw_arrays(mode, first, count) {
    gl_context.drawArrays(mode, first, count);
}
export function clear_color(r, g, b, a) {
    gl_context.clearColor(r, g, b, a);
}
export function clear(mask) {
    gl_context.clear(mask);
}

export function create_texture() {
    return gl_context.createTexture();
}
export function delete_texture(texture) {
    gl_context.deleteTexture(texture);
}
export function bind_texture(target, texture) {
    gl_context.bindTexture(target, texture);
};
export function active_texture(texture) {
    gl_context.activeTexture(texture);
}
export function tex_parameter_i(target, pname, param) {
    gl_context.texParameteri(target, pname, param);
}

function tex_format_type_size(format) {
    switch (format) {
        case WebGLRenderingContext.ALPHA:
            return 1;
            break;
        case WebGLRenderingContext.RGB:
            return 3;
            break;
        case WebGLRenderingContext.RGBA:
            return 4;
            break;
        case WebGLRenderingContext.LUMINANCE:
            return 4;
            break;
        case WebGLRenderingContext.RGBA:
            return 4;
            break;
        default:
            throw "unsupported format type";
    }
}
function tex_empty_array_buffer(width, height, format, data_type) {
    let format_size = tex_format_type_size(format);
    switch (data_type) {
        case WebGLRenderingContext.UNSIGNED_BYTE:
            return new Uint8Array(width * height * format_size);
            break;
        case WebGLRenderingContext.UNSIGNED_SHORT_5_6_5
            || WebGLRenderingContext.UNSIGNED_SHORT_4_4_4_4
            || WebGLRenderingContext.UNSIGNED_SHORT_5_5_5_1:
            return new Uint16Array(width * height * format_size);
            break;
        default:
            throw ("unsupported data_type")
    }
}
export function tex_image_2d_empty(target, level, internalFormat, width, height, border, format, data_type, data) {
    var pixels = tex_empty_array_buffer(width, height, format, data_type);
    gl_context.texImage2D(target, level, internalFormat, width, height, border, format, data_type, pixels);
}
export function tex_image_2d_u8(target, level, internalFormat, width, height, border, format, data_type, pixels) {
    if (data_type != WebGLRenderingContext.UNSIGNED_BYTE) {
        throw "invalid data type for Uint8Array data"
    }
    gl_context.texImage2D(target, level, internalFormat, width, height, border, format, data_type, pixels);
}
export function tex_image_2d_u16(target, level, internalFormat, width, height, border, format, data_type, pixels) {
    if (data_type == WebGLRenderingContext.UNSIGNED_BYTE) {
        throw "invalid data type for Uint16Array data"
    }
    gl_context.texImage2D(target, level, internalFormat, width, height, border, format, data_type, pixels);
}
export function tex_sub_image_2d_u8(target, level, xoffset, yoffset, width, height, format, data_type, pixels) {
    if (data_type != WebGLRenderingContext.UNSIGNED_BYTE) {
        throw "invalid data type for Uint8Array data"
    }
    if (data_type == WebGLRenderingContext.UNSIGNED_BYTE && format == WebGLRenderingContext.RGBA) {
        gl_context.texSubImage2D(target, level, xoffset, yoffset, width, height, format, data_type, pixels);
    } else {
        throw "Only format of RGBA and data type of UNSIGNED_BYTE supported"
    }
}
export function tex_sub_image_2d_u16(target, level, xoffset, yoffset, width, height, format, data_type, pixels) {
    if (data_type == WebGLRenderingContext.UNSIGNED_BYTE) {
        throw "invalid data type for Uint16Array data"
    }
    gl_context.texSubImage2D(target, level, xoffset, yoffset, width, height, format, data_type, pixels);
}

export function create_shader(type) {
    return gl_context.createShader(type);
}
export function delete_shader(shader) {
    gl_context.deleteShader(shader);
}
export function shader_source(shader, source) {
    gl_context.shaderSource(shader, source);
}
export function compile_shader(shader) {
    gl_context.compileShader(shader);
}
export function get_shader_parameter(shader, pname) {
    return gl_context.getShaderParameter(shader, pname);
};
export function shader_info_log_len(shader) {
    const utf8Encoder = new TextEncoder("UTF-8");
    gl_tring_buffer = utf8Encoder.encode(this.context.getShaderInfoLog(shader))
    return string_buffer.length
}
export function get_shader_info_log(shader, size, str_ptr) {
    const utf8Encoder = new TextEncoder("UTF-8");
    gl_tring_buffer = utf8Encoder.encode(this.context.getShaderInfoLog(shader))
    for (i = 0; i < size; i++) {
        HEAPU8[str_ptr + i] = string_buffer[i]
    }
}

export function create_program() {
    return gl_context.createProgram();
}
export function delete_program(program) {
    gl_context.deleteProgram(program);
}
export function attach_shader(program, shader) {
    gl_context.attachShader(program, shader);
}
export function link_program(program) {
    gl_context.linkProgram(program);
}
export function use_program(program) {
    gl_context.useProgram(program);
}
export function get_program_parameter(program, pname) {
    return gl_context.getProgramParameter(program, pname);
}
export function program_info_log_len(program) {
    gl_tring_buffer = utf8Encoder.encode(this.context.getProgramInfoLog(program))
    return string_buffer.length
}
export function get_program_info_log(program) {
    return gl_context.getProgramInfoLog(program);
}

export function create_buffer() {
    return gl_context.createBuffer();
}
export function delete_buffer(buffer) {
    gl_context.deleteBuffer(buffer);
}
export function bind_buffer(target, buffer) {
    gl_context.bindBuffer(target, buffer);
}
export function buffer_data(target, data, usage) {
    gl_context.bufferData(target, data, usage);
}

export function get_uniform_location(program, name) {
    return gl_context.getUniformLocation(program, name);
};
export function uniform2f(location, v0, v1) {
    gl_context.uniform2f(location, v0, v1);
}
export function uniform1i(location, v0) {
    gl_context.uniform1i(location, v0);
}

export function get_attrib_location(program, name) {
    return gl_context.getAttribLocation(program, name);
};
export function enable_vertex_attrib_array(index) {
    gl_context.enableVertexAttribArray(index);
}
export function vertex_attrib_pointer(index, size, type, normalized, stride, offset) {
    gl_context.vertexAttribPointer(index, size, type, normalized, stride, offset);
}