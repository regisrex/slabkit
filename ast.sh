Ok(Element(HtmlElement {
    tag: "section",
    attributes: {},
    children: [
        Element(HtmlElement {
            tag: "div",
            attributes: {"class": "container"},
            children: [
                Element(HtmlElement {
                    tag: "h1",
                    attributes: {},
                    children: [
                        Element(HtmlElement {
                            tag: "span",
                            attributes: {},
                            children: [
                                Text("!{section.title}!")
                            ]
                        }),
                        Text("name"),
                        Text("quick"),
                        Text("brown"),
                        Text("fox"),
                        Text("jumps")
                    ]
                }),
                Element(HtmlElement {
                    tag: "img",
                    attributes: {
                        "style": "font-family:!{section.fontFamily}!;",
                        "src": "!{person.photo}!",
                        "alt": "Empty"
                    },
                    children: []
                }),
                Element(HtmlElement {
                    tag: "p",
                    attributes: {},
                    children: [
                        Text("!{section.description}!")
                    ]
                })
            ]
        }),
        Element(HtmlElement {
            tag: "div",
            attributes: {},
            children: [
                Element(HtmlElement {
                    tag: "h1",
                    attributes: {},
                    children: [
                        Text("!{section.names.title}!")
                    ]
                }),
                Element(HtmlElement {
                    tag: "slk-datamap",
                    attributes: {
                        "data": "!{section.people}!",
                        "selector": "!{person}!"
                    },
                    children: [
                        Element(HtmlElement {
                            tag: "h1",
                            attributes: {},
                            children: [
                                Text("!{person.name}!")
                            ]
                        })
                    ]
                })
            ] 
        })
    ] 
}))