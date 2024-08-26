use crate::protocol::status;

pub struct StatusProtocol;

impl StatusProtocol {
    pub fn ping(&self, ping: status::client::Ping) -> status::server::Pong {
        let payload = ping.payload;
        status::server::Pong { payload }
    }

    pub fn status(&self, _: status::client::StatusRequest) -> status::server::StatusResponse {
        let json_response = "{\"version\":{\"name\":\"1.20.4\",\"protocol\":767},\"players\":{\"max\":20,\"online\":1,\"sample\":[{\"name\":\"zirkonya\",\"id\":\"2bf12816-3494-48de-b69f-95662f7d34c1\"}]},\"description\":{\"text\":\"My rusty server\"},\"favicon\":\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFAUlEQVR4nO1bPWskRxB9NmYEEpNdcsKgTFrBmQM1GJQNLM6sZDOBIwWHcl12sTP/AKFA0cFll1jpwWQHhhEICyxtsODESg4UDCPYSexgr0c1PdWf07MbeB8I7Ud/vVfV1TXdvcAaa6zxf8Y3q+z84ODgX91319fXSxnb0gQwkTVhaCGiN763t9ciurW11bvNIUWI2rBKXkUfMYYSIVqjNvIUoUIMIUKUBn3Im+AiTGwRejcWi7wKmxixhAhuxJd4Xdfa75IkYT9fhkcEVXYhbyLMoY8IQLgQ34ZUMqGu62jkAaCqKqc2QvMMb9V01vclDQCz2azz2Wg0YssO5QlehTnyvsQ50jqoYgwhgnPBUPKXl5et91mWuXbZwmg08sofXEUIjgEhLg8AeZ4jz3Pvend3d0H92eCkkmr9EMvb4OoZQogmMMbIFaILoCM+vdK3v/vz82sXIWhsMIkQRYC+5CnxzY0UT/PS2N/mRgoATbk3v2VsOSlCXy/wigG+8161OiUvidpw8TbHxdvcq18fGAWg1reRV60/vdKTPBRZxxM2N9Km/HgiOnXevyvw/l3RvJdBsaoq52SJw3fBNb/C5PY6d/9c5Ozn44nAlynfz9O8dPYaH2jnh8vct815DtTKv3/IXcbYqfvLrwsPyfMcp6enrTJcTDDFgajPAjry1HLSK75MgaPjzKldzvIhuQQHVoAQ60vydC5LPM1LHIqs9f7mtsCnjwVcIEWT/2ks6IsgDzAlOXKQlDDAz3vbqmCa87ap5oqOACE7PNxyd3NbGAmoAZILmJxAarnz83Pr+EyPylYPUN3fJ8XllrpQ2BKoUPQOgjpXtFmUQsYN+qeCBswQL9ChVx7gsuTRwapxoS6Bv/7mAxqXNh+KTJtDhCLaMqhb0qRFxW6GWnGAJAX2dwT2d7qZ3+tXoql7dJzh08cCL3b5vk9OTprXvplhFAGOjrNOBmea7z8cdz9Thbi5XXiGzA7HE9EsmzEzwt6pMIBORqcOkBKrS+DPD/q29ndEa1o0ucLtcxk6Naj1Q+DtAbYOTeQl6hKd6WCro4LuIfSBVQBuy9okwngiMJ6IJl83gYqwTYon6cLK3KpBVwnTOFxjQfRzgfJh8X96tXhoSbfd6v2jyW6pCK9f2UX1RS8BVDeUlikfgHkJPHwlpYog3rQtTpEY4psMjD6oqsr4NOgUBJMkcdoN+vHwmdWL3WdvUFFcaPrRkFf3Ar7/KV5WGOwB6vxTg59KPt3uegL9jpI3JTuSvOw/SRLj0ZoNUZZBn3XZFhNMTankY4CdG67nf7PZrHk2lwnLRgqkL/Xuz0HGOZ3laaxRd4BsuL+/j7crzEEudzRV5cinL/nXPuSB8BMpHZz3BHUDoIeddKeGBsR5ybv2H5/bUV09C1C3vajru8x7m/UBy8GIy4GoetqrbldRIShM5Ln9PnXe2wRwIQ8EnAwB/iK4gGaOVICzszMAwOPjI4B4xCWCYoA6CCHaVnZJg7ny6smxJK/rNwZ63Q8AuvvwRdG1vskjKHkJlTjgdlXG1/rAAAJIcELosCryQIQrMkA/ESR5jjhgJx9KXCKKAID9mJoTQ40dFENanSLaLTGJvrfDffbzViKARGwhfI+4Y5AHlnBX2EWIVZEHVnBbnAoScrEhJnlgRb8XCEFs4hJL/cVICIYiLjH4b4ZCMTRxiUE7CRFjWcTXWGONNQDgP+DcfkNgWGhTAAAAAElFTkSuQmCC\",\"enforcesSecureChat\":false,\"previewsChat\":false}";
        status::server::StatusResponse {
            json_response: json_response.to_string(),
        }
    }
}
