# Warning: this script will not create a workable source file, you need to tweak it after

attributes_target_path = ""
elements_target_path = ""

import os
import glob
import json
from dataclasses import dataclass

folder_path = "elements"

file_list = glob.glob(os.path.join(folder_path, '*'))

ignore_tags = ["html"]

@dataclass
class Element:
    has_closing_tag: bool
    content: any

@dataclass
class Attribute:
    ty: str
    content: any

    def __eq__(self, other):
        this_attr = self.content[1]
        other_attr = other.content[1]
        return this_attr == other_attr
    
    def __hash__(self):
        return hash(self.content[1])
    

attributes: set[Attribute] = set()
elements: list[Element] = []

def snake_to_pascal(snake_str):
    components = snake_str.split('_')
    pascal_str = ''.join(x.capitalize() for x in components)
    return pascal_str

for file_path in file_list:
    with open(file_path, 'r') as file:
        content = json.load(file)
        if content["tag_name"] not in ignore_tags:
            attrs = [attr["field_name"] for attr in content["attributes"]]
            elements.append(Element(content["has_closing_tag"], [content["tag_name"], content["mdn_link"], attrs]))

            for attr in content["attributes"]:
                attributes.add(Attribute(attr["ty"], [attr["field_name"], attr["name"], attr["description"]]))

with open(attributes_target_path, 'w') as file:
    for attr in attributes:
        if attr.ty == "String":
            macro_name = "declare_kv_attribute"
        elif attr.ty == "Bool":
            macro_name = "declare_bool_attribute"
        elif attr.ty == "Integer":
            macro_name = "declare_int_attribute"
        else:
            macro_name = "declare_float_attribute"
        file.write(f"{macro_name}!({attr.content[0]}, \"{attr.content[1]}\", r#\"{attr.content[2]}\"#);\n")

with open(elements_target_path, 'w') as file:
    for element in elements:
        if element.has_closing_tag:
            macro_name = "declare_element"
        else:
            macro_name = "declare_element_void"
        file.write(f"{macro_name}!({element.content[0]}, \"{element.content[1]}\", {", ".join([snake_to_pascal(attr) for attr in element.content[2]])});\n")