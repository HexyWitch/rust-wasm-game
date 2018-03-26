var gl_global;

window.get_canvas_context = function (canvas_id) {
    return document.getElementById(canvas_id).getContext('webgl');
}
window.set_global_context = function (context) {
    gl_global = context;
}


window.gl_enable = function (capability) {
    gl_global.enable(capability);
}
window.gl_blend_func = function (sfactor, dfactor) {
    gl_global.blendFunc(sfactor, dfactor);
}
window.gl_draw_arrays = function (mode, first, count) {
    gl_global.drawArrays(mode, first, count);
}
window.gl_clear_color = function (r, g, b, a) {
    gl_global.clearColor(r, g, b, a);
}
window.gl_clear = function (mask) {
    gl_global.clear(mask);
}

window.gl_drawing_buffer_width = function () {
    return gl_global.drawingBufferWidth;
}
window.gl_drawing_buffer_height = function () {
    return gl_global.drawingBufferHeight;
}

window.gl_create_texture = function () {
    return gl_global.createTexture();
}
window.gl_delete_texture = function (texture) {
    gl_global.deleteTexture(texture);
}
window.gl_bind_texture = function (target, texture) {
    gl_global.bindTexture(target, texture);
};
window.gl_active_texture = function (texture) {
    gl_global.activeTexture(texture);
}
window.gl_tex_parameter_i = function (target, pname, param) {
    gl_global.texParameteri(target, pname, param);
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
window.gl_tex_image_2d_empty = function (target, level, internalFormat, width, height, border, format, data_type, data) {
    var pixels = tex_empty_array_buffer(width, height, format, data_type);
    gl_global.texImage2D(target, level, internalFormat, width, height, border, format, data_type, pixels);
}
window.gl_tex_image_2d_u8 = function (target, level, internalFormat, width, height, border, format, data_type, pixels) {
    if (data_type != WebGLRenderingContext.UNSIGNED_BYTE) {
        throw "invalid data type for Uint8Array data"
    }
    gl_global.texImage2D(target, level, internalFormat, width, height, border, format, data_type, pixels);
}
window.gl_tex_image_2d_u16 = function (target, level, internalFormat, width, height, border, format, data_type, pixels) {
    if (data_type == WebGLRenderingContext.UNSIGNED_BYTE) {
        throw "invalid data type for Uint16Array data"
    }
    gl_global.texImage2D(target, level, internalFormat, width, height, border, format, data_type, pixels);
}
window.gl_tex_sub_image_2d_u8 = function (target, level, xoffset, yoffset, width, height, format, data_type, pixels) {
    if (data_type != WebGLRenderingContext.UNSIGNED_BYTE) {
        throw "invalid data type for Uint8Array data"
    }
    if (data_type == WebGLRenderingContext.UNSIGNED_BYTE && format == WebGLRenderingContext.RGBA) {
        gl_global.texSubImage2D(target, level, xoffset, yoffset, width, height, format, data_type, pixels);
    } else {
        throw "Only format of RGBA and data type of UNSIGNED_BYTE supported"
    }
}
window.gl_tex_sub_image_2d_u16 = function (target, level, xoffset, yoffset, width, height, format, data_type, pixels) {
    if (data_type == WebGLRenderingContext.UNSIGNED_BYTE) {
        throw "invalid data type for Uint16Array data"
    }
    gl_global.texSubImage2D(target, level, xoffset, yoffset, width, height, format, data_type, pixels);
}

window.gl_create_shader = function (type) {
    return gl_global.createShader(type);
}
window.gl_delete_shader = function (shader) {
    gl_global.deleteShader(shader);
}
window.gl_shader_source = function (shader, source) {
    gl_global.shaderSource(shader, source);
}
window.gl_compile_shader = function (shader) {
    gl_global.compileShader(shader);
}
window.gl_get_shader_parameter = function (shader, pname) {
    return gl_global.getShaderParameter(shader, pname);
};
window.gl_shader_info_log_len = function (shader) {
    const utf8Encoder = new TextEncoder("UTF-8");
    gl_tring_buffer = utf8Encoder.encode(this.context.getShaderInfoLog(shader))
    return string_buffer.length
}
window.gl_get_shader_info_log = function (shader, size, str_ptr) {
    const utf8Encoder = new TextEncoder("UTF-8");
    gl_tring_buffer = utf8Encoder.encode(this.context.getShaderInfoLog(shader))
    for (i = 0; i < size; i++) {
        HEAPU8[str_ptr + i] = string_buffer[i]
    }
}

window.gl_create_program = function () {
    return gl_global.createProgram();
}
window.gl_delete_program = function (program) {
    gl_global.deleteProgram(program);
}
window.gl_attach_shader = function (program, shader) {
    gl_global.attachShader(program, shader);
}
window.gl_link_program = function (program) {
    gl_global.linkProgram(program);
}
window.gl_use_program = function (program) {
    gl_global.useProgram(program);
}
window.gl_get_program_parameter = function (program, pname) {
    return gl_global.getProgramParameter(program, pname);
}
window.gl_program_info_log_len = function (program) {
    gl_tring_buffer = utf8Encoder.encode(this.context.getProgramInfoLog(program))
    return string_buffer.length
}
window.gl_get_program_info_log = function (program) {
    return gl_global.getProgramInfoLog(program);
}

window.gl_create_buffer = function () {
    return gl_global.createBuffer();
}
window.gl_delete_buffer = function (buffer) {
    gl_global.deleteBuffer(buffer);
}
window.gl_bind_buffer = function (target, buffer) {
    gl_global.bindBuffer(target, buffer);
}
window.gl_buffer_data = function (target, data, usage) {
    gl_global.bufferData(target, data, usage);
}

window.gl_get_uniform_location = function (program, name) {
    return gl_global.getUniformLocation(program, name);
};
window.gl_uniform2f = function (location, v0, v1) {
    gl_global.uniform2f(location, v0, v1);
}
window.gl_uniform1i = function (location, v0) {
    gl_global.uniform1i(location, v0);
}

window.gl_get_attrib_location = function (program, name) {
    return gl_global.getAttribLocation(program, name);
};
window.gl_enable_vertex_attrib_array = function (index) {
    gl_global.enableVertexAttribArray(index);
}
window.gl_vertex_attrib_pointer = function (index, size, type, normalized, stride, offset) {
    gl_global.vertexAttribPointer(index, size, type, normalized, stride, offset);
}