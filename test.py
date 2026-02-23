import ctypes
import json
import pprint
def query_data_javascript_qj():
    libObject = ctypes.WinDLL('./target/debug/dominodbc.dll')
    cdata = ctypes.c_char_p(b"C:/NHKI/domy/data/500_KB_V2.json")

    res=libObject.domino_load_data(cdata)

    with open("C:/NHKI/domy/query.txt", "r", encoding="utf-8") as file:
        content = file.read()
    
    bquery=content.encode()

    cquery = ctypes.c_char_p(bquery)
    #libObject.domino_load_data(cdata)
    nbuffer= ctypes.c_int32=1024*1000
    buffer = ctypes.create_string_buffer(nbuffer)
    res=libObject.domino_query(cquery,buffer,nbuffer)
    if res == 0:
        sjson=buffer.value.decode()
        ret=json.loads(sjson)
        print("Query with qjs")
        pprint.pprint(ret)
    else:
        serror=buffer.value.decode()
        print(serror)
        raise Exception(serror)

query_data_javascript_qj()