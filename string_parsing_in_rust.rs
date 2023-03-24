use std::{collections::HashMap, ops::Index};
mod r#try;
mod train;
// use std::collections::HashMap;
fn getkv(data: &str) -> HashMap<String,String> {
    let mut key_value_pairs = HashMap::new();
    let mut line_no = 1;
    for line in data.lines() {
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() == 2 {
            key_value_pairs.insert(line_no.to_string(), parts[1].to_string());
        }
        line_no += 1;
    }
    key_value_pairs
}
fn findkey(lwc:&String)->String{
    let mut ql=0;
    let mut startat=0;
    let mut endat=0;
    let mut lw=0;
    let mut string_to_work_with="".to_string();
    // let mut p=0;
    // println!("interim1----->{}",lwc);
    
    for (l,i) in lwc.chars().enumerate(){
        
        if i.eq(&'"')
        {
            // println!("interimk----->{}",lwc.chars().skip(l).into_iter().collect::<String>());
            ql+=1;
            if(ql==2){
                startat=l;
                string_to_work_with=lwc.chars().skip(l).skip(1).into_iter().collect::<String>();
                break;
                // lwc[p..].to_string();
            }         
         // break;
         }
     }
    //  println!("interim2----->{}",string_to_work_with);
     let mut res=vec![];
     for (p,g) in string_to_work_with.chars().enumerate(){
        // println!("interimu----->{}",string_to_work_with.chars().skip(p).into_iter().collect::<String>());
        if(!g.eq(&'"')){
            res.push(g);
            // print!("{g}")
        }
        else{
            break;
        }
     }

     let result =res.into_iter().collect::<String>();
     println!("{}", result);
     result
}
fn checkindex(lwc:&String,l:&usize)->usize{
    if(l+4)<lwc.len(){
        (l+4)
    }
    else{
        0
    }
}
fn dostuffwithline(mut lwc:&String)->String{
    // println!("{lwc}");
    let key_string="text".to_string();
    let key_string_length=key_string.len();
    let lwc=&lwc.chars().filter(|c| !c.is_whitespace()).into_iter().collect::<String>();
    // let text_vec = lwc.chars().collect::<Vec<_>>();
    let mut result="".to_string();
    // let mut p=0;
    for (l,i) in lwc.chars().enumerate(){
       if i.eq(&'t')
       {
        let tms=lwc.chars().skip(l).take(key_string_length).into_iter().collect::<String>();
        // println!("{}",tms);
        // if (l+4)<lwc.len()
        {

        if((tms)==key_string)
            {
                // p=;
                result = findkey(&lwc.chars().skip(l).into_iter().collect::<String>());
                break;
            }
        // break;
        }
    }
        
    }
result
    // println!("{}",lwc[p..].to_string())
}
#[test]
fn tryit(){
    dostuffwithline(&r#"whtaever is "text": "this" whgasdasdsa"#.to_string());
    // dostuffwithline(&r#"whtaever is "tIT“""#.to_string());
}
fn parse_data(data: &str) -> HashMap<String, String> {
    let mut key_value_pairs = HashMap::new();
    let mut line_no = 1;
    let mut all_line_with_context = String::new();
    for line in data.lines() {
        if line.contains("title")
        {
            let line_with_context = data.lines().skip(line_no - 3).take(5).filter(|line| contains_non_english(line)).collect::<Vec<&str>>().join("\n");
            // let value = printallnonenglish(&line_with_context).trim().to_string();
            let value = dostuffwithline(&line_with_context).trim().to_string();
            all_line_with_context.push_str(&line_with_context);
            key_value_pairs.insert(line_no.clone().to_string(), value);
        }
        line_no += 1;
    }
    // println!("{}", all_line_with_context);
    key_value_pairs
    // getkv(&all_line_with_context)
}

fn printallnonenglish(input_string:&String)->String{
    let english_chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
// let input_string = "sda \"text\":\"whatever\"\nsdasd  \"text\":\"sdfsdf\"\n asda\"text\":\"dsfsd\"\n\"text\":\"dsfs\" asdas";
let non_english_chars = input_string.chars().filter(|c| !english_chars.contains(*c)).collect::<String>();
println!("{}", non_english_chars);
non_english_chars
}

use regex::Regex;

// fn get_values(data: &str) -> HashMap<String, String> {
//     let mut key_value_pairs = HashMap::new();
//     let re = Regex::new(r"(?P<key>[a-zA-Z0-9_]+)=(?P<value>[^=\n]+)").unwrap();
//     let mut line_no = 1;
//     for capture in re.captures_iter(data) {
//         let value = capture.name("value").unwrap().as_str();
//         if !value.contains("{") && !value.contains("[") {
//             key_value_pairs.insert(line_no.to_string(), value.to_string());
//         }
//         line_no += 1;
//     }
//     key_value_pairs
// }
fn contains_non_english(text: &str) -> bool {
    let re = Regex::new(r"[^\p{ASCII}]").unwrap();
    re.is_match(text)
}
fn main(){
    print!("test")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_data() {
        let data=
        r#"
        var ytInitialData = {
            "responseContext": {
                "serviceTrackingParams": [{
                    "service": "GFEEDBACK",
                    "params": [{
                        "key": "route",
                        "value": "channel.videos"
                    }, {
                        "key": "is_casual",
                        "value": "false"
                    }, {
                        "key": "is_owner",
                        "value": "false"
                    }, {
                        "key": "is_monetization_enabled",
                        "value": "true"
                    }, {
                        "key": "num_shelves",
                        "value": "1"
                    }, {
                        "key": "is_alc_surface",
                        "value": "false"
                    }, {
                        "key": "browse_id",
                        "value": "UCmjdhwMGSut8mZ1CqnRjjUw"
                    }, {
                        "key": "browse_id_prefix",
                        "value": ""
                    }, {
                        "key": "logged_in",
                        "value": "0"
                    }, {
                        "key": "country-type",
                        "value": "B"
                    }, {
                        "key": "e",
                        "value": "1714246,23804281,23885487,23918597,23943577,23946420,23966208,23983296,23986029,23998056,24004644,24007246,24034168,24036947,24077241,24080738,24120819,24135310,24140247,24162920,24166867,24169501,24181174,24187043,24187377,24209350,24211178,24219713,24224266,24240253,24241378,24248091,24255543,24255545,24262346,24263796,24268142,24283093,24288043,24288664,24290971,24291857,24296352,24390675,24391541,24396645,24401012,24404640,24407191,24410609,24412855,24414717,24415864,24415866,24416291,24419347,24422005,24422532,24426636,24428415,24428900,24429092,24430079,24433679,24439361,24439483,24440132,24441787,24447992,24449113,24450367,24453129,24453989,24458839,24459435,24465011,24465095,24466371,24466460,24466828,24466833,24466859,24468724,24470718,24475539,24476841,24479966,24481771,24482080,24483178,24483504,24483961,24484079,24484375,24485239,24485410,24485420,24486576,24487461,24487523,24487565,24487928,24489232,24490202,24491070,24491630,24493736,24494052,24494077,24494197,24494986,24494993,24495060,24495403,24495841,24495878,24495962,24495967,24496739,24496969,24497672,24498726,24499287,24499298,24499570,24499577,24499792,24510799,24512908,24515424,39323074,39323341,39323453,45686551"
                    }]
                }, {
                    "service": "GOOGLE_HELP",
                    "params": [{
                        "key": "browse_id",
                        "value": "UCmjdhwMGSut8mZ1CqnRjjUw"
                    }, {
                        "key": "browse_id_prefix",
                        "value": ""
                    }]
                }, {
                    "service": "CSI",
                    "params": [{
                        "key": "c",
                        "value": "WEB"
                    }, {
                        "key": "cver",
                        "value": "2.20230320.07.00"
                    }, {
                        "key": "yt_li",
                        "value": "0"
                    }, {
                        "key": "GetChannelPage_rid",
                        "value": "0x8f71178fc611a5b1"
                    }]
                }, {
                    "service": "GUIDED_HELP",
                    "params": [{
                        "key": "logged_in",
                        "value": "0"
                    }]
                }, {
                    "service": "ECATCHER",
                    "params": [{
                        "key": "client.version",
                        "value": "2.20230320"
                    }, {
                        "key": "client.name",
                        "value": "WEB"
                    }, {
                        "key": "client.fexp",
                        "value": "24166867,24428415,24466371,24390675,24288664,24433679,24512908,24468724,24453989,39323453,24499792,24412855,24396645,24268142,24439361,24489232,24483961,24441787,24495841,24466828,24483178,24485239,24169501,24407191,24290971,24453129,24458839,24415864,24162920,24499570,24080738,24459435,24077241,24291857,24219713,23983296,24495967,24209350,24484375,24499577,24004644,24491630,24494197,24475539,24248091,23804281,23946420,24240253,24135310,24007246,24296352,24414717,23918597,24483504,24428900,24470718,24496739,24255543,39323341,23966208,24487461,24140247,39323074,24466859,24288043,24494077,24485410,24187377,23986029,24476841,24487928,24486576,24495060,24450367,24510799,24283093,24494052,24466833,24416291,24449113,24493736,24498726,24262346,24494986,24495878,24263796,24484079,24515424,24410609,24391541,23998056,24036947,24494993,24187043,23943577,24426636,24465095,24224266,24401012,24415866,24499298,24495962,24497672,24241378,24485420,24465011,24181174,24404640,24034168,24491070,24211178,24429092,45686551,24499287,24447992,24419347,24439483,1714246,24481771,24120819,24482080,24466460,23885487,24430079,24479966,24487523,24422532,24495403,24487565,24440132,24496969,24422005,24490202,24255545"
                    }]
                }],
                "maxAgeSeconds": 300,
                "mainAppWebResponseContext": {
                    "loggedOut": true
                },
                "webResponseContextExtensionData": {
                    "ytConfigData": {
                        "visitorData": "CgtjVUxud3AyMkZvbyjM0PCgBg%3D%3D",
                        "rootVisualElementType": 3611
                    },
                    "hasDecorated": true
                }
            },
            "contents": {
                "twoColumnBrowseResultsRenderer": {
                    "tabs": [{
                        "tabRenderer": {
                            "endpoint": {
                                "clickTrackingParams": "CNEBEPCTARgFIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                "commandMetadata": {
                                    "webCommandMetadata": {
                                        "url": "/@xiaopeng163/featured",
                                        "webPageType": "WEB_PAGE_TYPE_CHANNEL",
                                        "rootVe": 3611,
                                        "apiUrl": "/youtubei/v1/browse"
                                    }
                                },
                                "browseEndpoint": {
                                    "browseId": "UCmjdhwMGSut8mZ1CqnRjjUw",
                                    "params": "EghmZWF0dXJlZPIGBAoCMgA%3D",
                                    "canonicalBaseUrl": "/@xiaopeng163"
                                }
                            },
                            "title": "Home",
                            "trackingParams": "CNEBEPCTARgFIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                        }
                    }, {
                        "tabRenderer": {
                            "endpoint": {
                                "clickTrackingParams": "CBgQ8JMBGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                "commandMetadata": {
                                    "webCommandMetadata": {
                                        "url": "/@xiaopeng163/videos",
                                        "webPageType": "WEB_PAGE_TYPE_CHANNEL",
                                        "rootVe": 3611,
                                        "apiUrl": "/youtubei/v1/browse"
                                    }
                                },
                                "browseEndpoint": {
                                    "browseId": "UCmjdhwMGSut8mZ1CqnRjjUw",
                                    "params": "EgZ2aWRlb3PyBgQKAjoA",
                                    "canonicalBaseUrl": "/@xiaopeng163"
                                }
                            },
                            "title": "Videos",
                            "selected": true,
                            "content": {
                                "richGridRenderer": {
                                    "contents": [{
                                        "richItemRenderer": {
                                            "content": {
                                                "videoRenderer": {
                                                    "videoId": "USkaFV6q5Kg",
                                                    "thumbnail": {
                                                        "thumbnails": [{
                                                            "url": "https://i.ytimg.com/vi/USkaFV6q5Kg/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLCD2B3iS8J2haJcVL_c0F_8Npjq0w",
                                                            "width": 168,
                                                            "height": 94
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/USkaFV6q5Kg/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLCK4vqwaxKnXDgkud4ZG71mifk7VA",
                                                            "width": 196,
                                                            "height": 110
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/USkaFV6q5Kg/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLD6jqswf6qrp7YcaGF21xFvWqh26g",
                                                            "width": 246,
                                                            "height": 138
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/USkaFV6q5Kg/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLCLqptBqgNlxOCLdL1BlQQSeoKvrw",
                                                            "width": 336,
                                                            "height": 188
                                                        }]
                                                    },
                                                    "title": {
                                                        "runs": [{
                                                            "text": "Git config详解  | 看看chatgpt怎么说"
                                                        }],
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "Git config详解  | 看看chatgpt怎么说 by 麦兜搞IT 5 days ago 7 minutes, 42 seconds 195 views"
                                                            }
                                                        }
                                                    },
                                                    "descriptionSnippet": {
                                                        "runs": [{
                                                            "text": "===============================\n【我的一些在线课程】\n\nUdemy：https://github.com/udemy-course\nHiskio：https://hiskio.com/search?word=peng%20xiao\u0026sort=attendance\u0026page=1\n\n【联系方式..."
                                                        }]
                                                    },
                                                    "publishedTimeText": {
                                                        "simpleText": "5 days ago"
                                                    },
                                                    "lengthText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "7 minutes, 42 seconds"
                                                            }
                                                        },
                                                        "simpleText": "7:42"
                                                    },
                                                    "viewCountText": {
                                                        "simpleText": "195 views"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CMwBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28WhhVQ21qZGh3TUdTdXQ4bVoxQ3FuUmpqVXeaAQYQ8jgY4AeqARpVVUxGbWpkaHdNR1N1dDhtWjFDcW5SampVdw==",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "url": "/watch?v=USkaFV6q5Kg",
                                                                "webPageType": "WEB_PAGE_TYPE_WATCH",
                                                                "rootVe": 3832
                                                            }
                                                        },
                                                        "watchEndpoint": {
                                                            "videoId": "USkaFV6q5Kg",
                                                            "watchEndpointSupportedOnesieConfig": {
                                                                "html5PlaybackOnesieConfig": {
                                                                    "commonConfig": {
                                                                        "url": "https://rr2---sn-cnoa-jv3s.googlevideo.com/initplayback?source=youtube\u0026oeis=1\u0026c=WEB\u0026oad=3200\u0026ovd=3200\u0026oaad=11000\u0026oavd=11000\u0026ocs=700\u0026oewis=1\u0026oputc=1\u0026ofpcc=1\u0026msp=1\u0026odepv=1\u0026id=51291a155eaae4a8\u0026ip=117.213.17.165\u0026initcwndbps=977500\u0026mt=1679566621\u0026oweuc=\u0026pxtags=Cg4KAnR4EggyNDQyODQxNQ\u0026rxtags=Cg4KAnR4EggyNDQyODQxNA%2CCg4KAnR4EggyNDQyODQxNQ%2CCg4KAnR4EggyNDQyODQxNg%2CCg4KAnR4EggyNDQyODQxNw%2CCg4KAnR4EggyNDQyODQxOA%2CCg4KAnR4EggyNDQyODQxOQ%2CCg4KAnR4EggyNDQyODQyMA"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CMwBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28QKjJq_XVwsaUUaoBGlVVTEZtamRod01HU3V0OG1aMUNxblJqalV3",
                                                    "showActionMenu": false,
                                                    "shortViewCountText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "195 views"
                                                            }
                                                        },
                                                        "simpleText": "195 views"
                                                    },
                                                    "menu": {
                                                        "menuRenderer": {
                                                            "items": [{
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Add to queue"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "ADD_TO_QUEUE_TAIL"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CNABEP6YBBgFIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true
                                                                            }
                                                                        },
                                                                        "signalServiceEndpoint": {
                                                                            "signal": "CLIENT_SIGNAL",
                                                                            "actions": [{
                                                                                "clickTrackingParams": "CNABEP6YBBgFIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "addToPlaylistCommand": {
                                                                                    "openMiniplayer": true,
                                                                                    "videoId": "USkaFV6q5Kg",
                                                                                    "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                                    "onCreateListCommand": {
                                                                                        "clickTrackingParams": "CNABEP6YBBgFIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                        "commandMetadata": {
                                                                                            "webCommandMetadata": {
                                                                                                "sendPost": true,
                                                                                                "apiUrl": "/youtubei/v1/playlist/create"
                                                                                            }
                                                                                        },
                                                                                        "createPlaylistServiceEndpoint": {
                                                                                            "videoIds": ["USkaFV6q5Kg"],
                                                                                            "params": "CAQ%3D"
                                                                                        }
                                                                                    },
                                                                                    "videoIds": ["USkaFV6q5Kg"]
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CNABEP6YBBgFIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                                }
                                                            }, {
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Share"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "SHARE"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CMwBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true,
                                                                                "apiUrl": "/youtubei/v1/share/get_share_panel"
                                                                            }
                                                                        },
                                                                        "shareEntityServiceEndpoint": {
                                                                            "serializedShareEntity": "CgtVU2thRlY2cTVLZw%3D%3D",
                                                                            "commands": [{
                                                                                "clickTrackingParams": "CMwBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "openPopupAction": {
                                                                                    "popup": {
                                                                                        "unifiedSharePanelRenderer": {
                                                                                            "trackingParams": "CM8BEI5iIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                            "showLoadingSpinner": true
                                                                                        }
                                                                                    },
                                                                                    "popupType": "DIALOG",
                                                                                    "beReused": true
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CMwBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                                }
                                                            }],
                                                            "trackingParams": "CMwBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                            "accessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Action menu"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "thumbnailOverlays": [{
                                                        "thumbnailOverlayTimeStatusRenderer": {
                                                            "text": {
                                                                "accessibility": {
                                                                    "accessibilityData": {
                                                                        "label": "7 minutes, 42 seconds"
                                                                    }
                                                                },
                                                                "simpleText": "7:42"
                                                            },
                                                            "style": "DEFAULT"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "isToggled": false,
                                                            "untoggledIcon": {
                                                                "iconType": "WATCH_LATER"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "CHECK"
                                                            },
                                                            "untoggledTooltip": "Watch later",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CM4BEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "addedVideoId": "USkaFV6q5Kg",
                                                                        "action": "ACTION_ADD_VIDEO"
                                                                    }]
                                                                }
                                                            },
                                                            "toggledServiceEndpoint": {
                                                                "clickTrackingParams": "CM4BEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "action": "ACTION_REMOVE_VIDEO_BY_VIDEO_ID",
                                                                        "removedVideoId": "USkaFV6q5Kg"
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Watch later"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CM4BEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "untoggledIcon": {
                                                                "iconType": "ADD_TO_QUEUE_TAIL"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "PLAYLIST_ADD_CHECK"
                                                            },
                                                            "untoggledTooltip": "Add to queue",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CM0BEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true
                                                                    }
                                                                },
                                                                "signalServiceEndpoint": {
                                                                    "signal": "CLIENT_SIGNAL",
                                                                    "actions": [{
                                                                        "clickTrackingParams": "CM0BEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "addToPlaylistCommand": {
                                                                            "openMiniplayer": true,
                                                                            "videoId": "USkaFV6q5Kg",
                                                                            "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                            "onCreateListCommand": {
                                                                                "clickTrackingParams": "CM0BEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "commandMetadata": {
                                                                                    "webCommandMetadata": {
                                                                                        "sendPost": true,
                                                                                        "apiUrl": "/youtubei/v1/playlist/create"
                                                                                    }
                                                                                },
                                                                                "createPlaylistServiceEndpoint": {
                                                                                    "videoIds": ["USkaFV6q5Kg"],
                                                                                    "params": "CAQ%3D"
                                                                                }
                                                                            },
                                                                            "videoIds": ["USkaFV6q5Kg"]
                                                                        }
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Add to queue"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CM0BEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayNowPlayingRenderer": {
                                                            "text": {
                                                                "runs": [{
                                                                    "text": "Now playing"
                                                                }]
                                                            }
                                                        }
                                                    }]
                                                }
                                            },
                                            "trackingParams": "CMsBEJmNBRgAIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                        }
                                    }, {
                                        "richItemRenderer": {
                                            "content": {
                                                "videoRenderer": {
                                                    "videoId": "KdZeLHECwsI",
                                                    "thumbnail": {
                                                        "thumbnails": [{
                                                            "url": "https://i.ytimg.com/vi/KdZeLHECwsI/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLCYh6S9CBABU5Ogq-qgg8-ChFBRpg",
                                                            "width": 168,
                                                            "height": 94
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/KdZeLHECwsI/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLC9r_BKUaEfGOPZo2EmiqDUKMBlVQ",
                                                            "width": 196,
                                                            "height": 110
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/KdZeLHECwsI/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLAvs3r8P68AlYfrKySPA43d5-obNQ",
                                                            "width": 246,
                                                            "height": 138
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/KdZeLHECwsI/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLAhjXsW7DnEHKivCzrPLFz28Kd6nQ",
                                                            "width": 336,
                                                            "height": 188
                                                        }]
                                                    },
                                                    "title": {
                                                        "runs": [{
                                                            "text": "一个叫Cursor的IDE可以让你“白嫖”GitHub Copilot | 大家赶紧去试试吧"
                                                        }],
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "一个叫Cursor的IDE可以让你“白嫖”GitHub Copilot | 大家赶紧去试试吧 by 麦兜搞IT 5 days ago 3 minutes, 59 seconds 2,628 views"
                                                            }
                                                        }
                                                    },
                                                    "descriptionSnippet": {
                                                        "runs": [{
                                                            "text": "https://www.cursor.so/\n\n===============================\n【我的一些在线课程】\n\nUdemy：https://github.com/udemy-course\nHiskio：https://hiskio.com/search?word=peng%20xiao\u0026sort=attendance\u0026p..."
                                                        }]
                                                    },
                                                    "publishedTimeText": {
                                                        "simpleText": "5 days ago"
                                                    },
                                                    "lengthText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "3 minutes, 59 seconds"
                                                            }
                                                        },
                                                        "simpleText": "3:59"
                                                    },
                                                    "viewCountText": {
                                                        "simpleText": "2,628 views"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CMYBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28WhhVQ21qZGh3TUdTdXQ4bVoxQ3FuUmpqVXeaAQYQ8jgY4AeqARpVVUxGbWpkaHdNR1N1dDhtWjFDcW5SampVdw==",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "url": "/watch?v=KdZeLHECwsI",
                                                                "webPageType": "WEB_PAGE_TYPE_WATCH",
                                                                "rootVe": 3832
                                                            }
                                                        },
                                                        "watchEndpoint": {
                                                            "videoId": "KdZeLHECwsI",
                                                            "watchEndpointSupportedOnesieConfig": {
                                                                "html5PlaybackOnesieConfig": {
                                                                    "commonConfig": {
                                                                        "url": "https://rr5---sn-cnoa-jv3s.googlevideo.com/initplayback?source=youtube\u0026oeis=1\u0026c=WEB\u0026oad=3200\u0026ovd=3200\u0026oaad=11000\u0026oavd=11000\u0026ocs=700\u0026oewis=1\u0026oputc=1\u0026ofpcc=1\u0026msp=1\u0026odepv=1\u0026id=29d65e2c7102c2c2\u0026ip=117.213.17.165\u0026initcwndbps=977500\u0026mt=1679566621\u0026oweuc=\u0026pxtags=Cg4KAnR4EggyNDQyODQxNQ\u0026rxtags=Cg4KAnR4EggyNDQyODQxNA%2CCg4KAnR4EggyNDQyODQxNQ%2CCg4KAnR4EggyNDQyODQxNg%2CCg4KAnR4EggyNDQyODQxNw%2CCg4KAnR4EggyNDQyODQxOA%2CCg4KAnR4EggyNDQyODQxOQ%2CCg4KAnR4EggyNDQyODQyMA"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CMYBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28QMKFi4jHxZfrKaoBGlVVTEZtamRod01HU3V0OG1aMUNxblJqalV3",
                                                    "showActionMenu": false,
                                                    "shortViewCountText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "2.6K views"
                                                            }
                                                        },
                                                        "simpleText": "2.6K views"
                                                    },
                                                    "menu": {
                                                        "menuRenderer": {
                                                            "items": [{
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Add to queue"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "ADD_TO_QUEUE_TAIL"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CMoBEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true
                                                                            }
                                                                        },
                                                                        "signalServiceEndpoint": {
                                                                            "signal": "CLIENT_SIGNAL",
                                                                            "actions": [{
                                                                                "clickTrackingParams": "CMoBEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "addToPlaylistCommand": {
                                                                                    "openMiniplayer": true,
                                                                                    "videoId": "KdZeLHECwsI",
                                                                                    "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                                    "onCreateListCommand": {
                                                                                        "clickTrackingParams": "CMoBEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                        "commandMetadata": {
                                                                                            "webCommandMetadata": {
                                                                                                "sendPost": true,
                                                                                                "apiUrl": "/youtubei/v1/playlist/create"
                                                                                            }
                                                                                        },
                                                                                        "createPlaylistServiceEndpoint": {
                                                                                            "videoIds": ["KdZeLHECwsI"],
                                                                                            "params": "CAQ%3D"
                                                                                        }
                                                                                    },
                                                                                    "videoIds": ["KdZeLHECwsI"]
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CMoBEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                                }
                                                            }, {
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Share"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "SHARE"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CMYBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true,
                                                                                "apiUrl": "/youtubei/v1/share/get_share_panel"
                                                                            }
                                                                        },
                                                                        "shareEntityServiceEndpoint": {
                                                                            "serializedShareEntity": "CgtLZFplTEhFQ3dzSQ%3D%3D",
                                                                            "commands": [{
                                                                                "clickTrackingParams": "CMYBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "openPopupAction": {
                                                                                    "popup": {
                                                                                        "unifiedSharePanelRenderer": {
                                                                                            "trackingParams": "CMkBEI5iIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                            "showLoadingSpinner": true
                                                                                        }
                                                                                    },
                                                                                    "popupType": "DIALOG",
                                                                                    "beReused": true
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CMYBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                                }
                                                            }],
                                                            "trackingParams": "CMYBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                            "accessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Action menu"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "thumbnailOverlays": [{
                                                        "thumbnailOverlayTimeStatusRenderer": {
                                                            "text": {
                                                                "accessibility": {
                                                                    "accessibilityData": {
                                                                        "label": "3 minutes, 59 seconds"
                                                                    }
                                                                },
                                                                "simpleText": "3:59"
                                                            },
                                                            "style": "DEFAULT"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "isToggled": false,
                                                            "untoggledIcon": {
                                                                "iconType": "WATCH_LATER"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "CHECK"
                                                            },
                                                            "untoggledTooltip": "Watch later",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CMgBEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "addedVideoId": "KdZeLHECwsI",
                                                                        "action": "ACTION_ADD_VIDEO"
                                                                    }]
                                                                }
                                                            },
                                                            "toggledServiceEndpoint": {
                                                                "clickTrackingParams": "CMgBEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "action": "ACTION_REMOVE_VIDEO_BY_VIDEO_ID",
                                                                        "removedVideoId": "KdZeLHECwsI"
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Watch later"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CMgBEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "untoggledIcon": {
                                                                "iconType": "ADD_TO_QUEUE_TAIL"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "PLAYLIST_ADD_CHECK"
                                                            },
                                                            "untoggledTooltip": "Add to queue",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CMcBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true
                                                                    }
                                                                },
                                                                "signalServiceEndpoint": {
                                                                    "signal": "CLIENT_SIGNAL",
                                                                    "actions": [{
                                                                        "clickTrackingParams": "CMcBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "addToPlaylistCommand": {
                                                                            "openMiniplayer": true,
                                                                            "videoId": "KdZeLHECwsI",
                                                                            "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                            "onCreateListCommand": {
                                                                                "clickTrackingParams": "CMcBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "commandMetadata": {
                                                                                    "webCommandMetadata": {
                                                                                        "sendPost": true,
                                                                                        "apiUrl": "/youtubei/v1/playlist/create"
                                                                                    }
                                                                                },
                                                                                "createPlaylistServiceEndpoint": {
                                                                                    "videoIds": ["KdZeLHECwsI"],
                                                                                    "params": "CAQ%3D"
                                                                                }
                                                                            },
                                                                            "videoIds": ["KdZeLHECwsI"]
                                                                        }
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Add to queue"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CMcBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayNowPlayingRenderer": {
                                                            "text": {
                                                                "runs": [{
                                                                    "text": "Now playing"
                                                                }]
                                                            }
                                                        }
                                                    }],
                                                    "richThumbnail": {
                                                        "movingThumbnailRenderer": {
                                                            "movingThumbnailDetails": {
                                                                "thumbnails": [{
                                                                    "url": "https://i.ytimg.com/an_webp/KdZeLHECwsI/mqdefault_6s.webp?du=3000\u0026sqp=COzI8KAG\u0026rs=AOn4CLABVgxKNpSiAv3W4jqO_kZDsqcbUA",
                                                                    "width": 320,
                                                                    "height": 180
                                                                }],
                                                                "logAsMovingThumbnail": true
                                                            },
                                                            "enableHoveredLogging": true,
                                                            "enableOverlay": true
                                                        }
                                                    }
                                                }
                                            },
                                            "trackingParams": "CMUBEJmNBRgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                        }
                                    }, {
                                        "richItemRenderer": {
                                            "content": {
                                                "videoRenderer": {
                                                    "videoId": "1epfoug8ixQ",
                                                    "thumbnail": {
                                                        "thumbnails": [{
                                                            "url": "https://i.ytimg.com/vi/1epfoug8ixQ/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLDwf5G-Eg1wMlUphaOEqVKyiGMggQ",
                                                            "width": 168,
                                                            "height": 94
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/1epfoug8ixQ/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLChWEjD7kWc6H0M19gNF0UAY7zP1Q",
                                                            "width": 196,
                                                            "height": 110
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/1epfoug8ixQ/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLA3dCdQ4YTdWVanHii0RTeQtJ-v7A",
                                                            "width": 246,
                                                            "height": 138
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/1epfoug8ixQ/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLB-0tQUrjtO4vjGcuWL5tTeRxpfPw",
                                                            "width": 336,
                                                            "height": 188
                                                        }]
                                                    },
                                                    "title": {
                                                        "runs": [{
                                                            "text": "GitHub Codespace使用一个月的体验 | 非常好用的Python模版 | 可以连ChatGPT API？"
                                                        }],
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "GitHub Codespace使用一个月的体验 | 非常好用的Python模版 | 可以连ChatGPT API？ by 麦兜搞IT 12 days ago 17 minutes 206 views"
                                                            }
                                                        }
                                                    },
                                                    "descriptionSnippet": {
                                                        "runs": [{
                                                            "text": "视频中使用的模版 https://github.com/xiaopeng163/python-devcontainer\n\n===============================\n【我的一些在线课程】\n\nUdemy：https://github.com/udemy-course\n\n【联系方式..."
                                                        }]
                                                    },
                                                    "publishedTimeText": {
                                                        "simpleText": "12 days ago"
                                                    },
                                                    "lengthText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "17 minutes, 39 seconds"
                                                            }
                                                        },
                                                        "simpleText": "17:39"
                                                    },
                                                    "viewCountText": {
                                                        "simpleText": "206 views"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CMABENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28WhhVQ21qZGh3TUdTdXQ4bVoxQ3FuUmpqVXeaAQYQ8jgY4AeqARpVVUxGbWpkaHdNR1N1dDhtWjFDcW5SampVdw==",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "url": "/watch?v=1epfoug8ixQ",
                                                                "webPageType": "WEB_PAGE_TYPE_WATCH",
                                                                "rootVe": 3832
                                                            }
                                                        },
                                                        "watchEndpoint": {
                                                            "videoId": "1epfoug8ixQ",
                                                            "watchEndpointSupportedOnesieConfig": {
                                                                "html5PlaybackOnesieConfig": {
                                                                    "commonConfig": {
                                                                        "url": "https://rr1---sn-cnoa-jv3s.googlevideo.com/initplayback?source=youtube\u0026oeis=1\u0026c=WEB\u0026oad=3200\u0026ovd=3200\u0026oaad=11000\u0026oavd=11000\u0026ocs=700\u0026oewis=1\u0026oputc=1\u0026ofpcc=1\u0026msp=1\u0026odepv=1\u0026id=d5ea5fa2e83c8b14\u0026ip=117.213.17.165\u0026initcwndbps=977500\u0026mt=1679566621\u0026oweuc=\u0026pxtags=Cg4KAnR4EggyNDQyODQxNQ\u0026rxtags=Cg4KAnR4EggyNDQyODQxNA%2CCg4KAnR4EggyNDQyODQxNQ%2CCg4KAnR4EggyNDQyODQxNg%2CCg4KAnR4EggyNDQyODQxNw%2CCg4KAnR4EggyNDQyODQxOA%2CCg4KAnR4EggyNDQyODQxOQ%2CCg4KAnR4EggyNDQyODQyMA"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CMABENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28QJSW8sGu9Jf11QGqARpVVUxGbWpkaHdNR1N1dDhtWjFDcW5SampVdw==",
                                                    "showActionMenu": false,
                                                    "shortViewCountText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "206 views"
                                                            }
                                                        },
                                                        "simpleText": "206 views"
                                                    },
                                                    "menu": {
                                                        "menuRenderer": {
                                                            "items": [{
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Add to queue"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "ADD_TO_QUEUE_TAIL"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CMQBEP6YBBgFIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true
                                                                            }
                                                                        },
                                                                        "signalServiceEndpoint": {
                                                                            "signal": "CLIENT_SIGNAL",
                                                                            "actions": [{
                                                                                "clickTrackingParams": "CMQBEP6YBBgFIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "addToPlaylistCommand": {
                                                                                    "openMiniplayer": true,
                                                                                    "videoId": "1epfoug8ixQ",
                                                                                    "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                                    "onCreateListCommand": {
                                                                                        "clickTrackingParams": "CMQBEP6YBBgFIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                        "commandMetadata": {
                                                                                            "webCommandMetadata": {
                                                                                                "sendPost": true,
                                                                                                "apiUrl": "/youtubei/v1/playlist/create"
                                                                                            }
                                                                                        },
                                                                                        "createPlaylistServiceEndpoint": {
                                                                                            "videoIds": ["1epfoug8ixQ"],
                                                                                            "params": "CAQ%3D"
                                                                                        }
                                                                                    },
                                                                                    "videoIds": ["1epfoug8ixQ"]
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CMQBEP6YBBgFIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                                }
                                                            }, {
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Share"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "SHARE"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CMABENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true,
                                                                                "apiUrl": "/youtubei/v1/share/get_share_panel"
                                                                            }
                                                                        },
                                                                        "shareEntityServiceEndpoint": {
                                                                            "serializedShareEntity": "CgsxZXBmb3VnOGl4UQ%3D%3D",
                                                                            "commands": [{
                                                                                "clickTrackingParams": "CMABENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "openPopupAction": {
                                                                                    "popup": {
                                                                                        "unifiedSharePanelRenderer": {
                                                                                            "trackingParams": "CMMBEI5iIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                            "showLoadingSpinner": true
                                                                                        }
                                                                                    },
                                                                                    "popupType": "DIALOG",
                                                                                    "beReused": true
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CMABENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                                }
                                                            }],
                                                            "trackingParams": "CMABENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                            "accessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Action menu"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "thumbnailOverlays": [{
                                                        "thumbnailOverlayTimeStatusRenderer": {
                                                            "text": {
                                                                "accessibility": {
                                                                    "accessibilityData": {
                                                                        "label": "17 minutes, 39 seconds"
                                                                    }
                                                                },
                                                                "simpleText": "17:39"
                                                            },
                                                            "style": "DEFAULT"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "isToggled": false,
                                                            "untoggledIcon": {
                                                                "iconType": "WATCH_LATER"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "CHECK"
                                                            },
                                                            "untoggledTooltip": "Watch later",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CMIBEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "addedVideoId": "1epfoug8ixQ",
                                                                        "action": "ACTION_ADD_VIDEO"
                                                                    }]
                                                                }
                                                            },
                                                            "toggledServiceEndpoint": {
                                                                "clickTrackingParams": "CMIBEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "action": "ACTION_REMOVE_VIDEO_BY_VIDEO_ID",
                                                                        "removedVideoId": "1epfoug8ixQ"
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Watch later"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CMIBEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "untoggledIcon": {
                                                                "iconType": "ADD_TO_QUEUE_TAIL"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "PLAYLIST_ADD_CHECK"
                                                            },
                                                            "untoggledTooltip": "Add to queue",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CMEBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true
                                                                    }
                                                                },
                                                                "signalServiceEndpoint": {
                                                                    "signal": "CLIENT_SIGNAL",
                                                                    "actions": [{
                                                                        "clickTrackingParams": "CMEBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "addToPlaylistCommand": {
                                                                            "openMiniplayer": true,
                                                                            "videoId": "1epfoug8ixQ",
                                                                            "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                            "onCreateListCommand": {
                                                                                "clickTrackingParams": "CMEBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "commandMetadata": {
                                                                                    "webCommandMetadata": {
                                                                                        "sendPost": true,
                                                                                        "apiUrl": "/youtubei/v1/playlist/create"
                                                                                    }
                                                                                },
                                                                                "createPlaylistServiceEndpoint": {
                                                                                    "videoIds": ["1epfoug8ixQ"],
                                                                                    "params": "CAQ%3D"
                                                                                }
                                                                            },
                                                                            "videoIds": ["1epfoug8ixQ"]
                                                                        }
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Add to queue"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CMEBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayNowPlayingRenderer": {
                                                            "text": {
                                                                "runs": [{
                                                                    "text": "Now playing"
                                                                }]
                                                            }
                                                        }
                                                    }]
                                                }
                                            },
                                            "trackingParams": "CL8BEJmNBRgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                        }
                                    }, {
                                        "richItemRenderer": {
                                            "content": {
                                                "videoRenderer": {
                                                    "videoId": "PnIrVDIcbGk",
                                                    "thumbnail": {
                                                        "thumbnails": [{
                                                            "url": "https://i.ytimg.com/vi/PnIrVDIcbGk/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLCBxiyDf0TRIc_7n5lOHfZgaDO2Rw",
                                                            "width": 168,
                                                            "height": 94
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/PnIrVDIcbGk/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLBAPMNbucZEcQa6GSqziW731AlzcA",
                                                            "width": 196,
                                                            "height": 110
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/PnIrVDIcbGk/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLBde5RQR8LKAeFlf-H7xVH2c3dwSA",
                                                            "width": 246,
                                                            "height": 138
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/PnIrVDIcbGk/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLCUQCVqd75HUXxR_13XSRCKtBzl2A",
                                                            "width": 336,
                                                            "height": 188
                                                        }]
                                                    },
                                                    "title": {
                                                        "runs": [{
                                                            "text": "介绍一个特别好用的 git pre commit工具"
                                                        }],
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "介绍一个特别好用的 git pre commit工具 by 麦兜搞IT 2 weeks ago 10 minutes, 53 seconds 190 views"
                                                            }
                                                        }
                                                    },
                                                    "descriptionSnippet": {
                                                        "runs": [{
                                                            "text": "demo project https://github.com/xiaopeng163/fastapi-mongodb-demo\npre-commit https://pre-commit.com/\n\n===============================\n【我的一些在线课程】\n\nUdemy：https://github.com/udemy..."
                                                        }]
                                                    },
                                                    "publishedTimeText": {
                                                        "simpleText": "2 weeks ago"
                                                    },
                                                    "lengthText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "10 minutes, 53 seconds"
                                                            }
                                                        },
                                                        "simpleText": "10:53"
                                                    },
                                                    "viewCountText": {
                                                        "simpleText": "190 views"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CLoBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28WhhVQ21qZGh3TUdTdXQ4bVoxQ3FuUmpqVXeaAQYQ8jgY4AeqARpVVUxGbWpkaHdNR1N1dDhtWjFDcW5SampVdw==",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "url": "/watch?v=PnIrVDIcbGk",
                                                                "webPageType": "WEB_PAGE_TYPE_WATCH",
                                                                "rootVe": 3832
                                                            }
                                                        },
                                                        "watchEndpoint": {
                                                            "videoId": "PnIrVDIcbGk",
                                                            "watchEndpointSupportedOnesieConfig": {
                                                                "html5PlaybackOnesieConfig": {
                                                                    "commonConfig": {
                                                                        "url": "https://rr3---sn-cnoa-jv3s.googlevideo.com/initplayback?source=youtube\u0026oeis=1\u0026c=WEB\u0026oad=3200\u0026ovd=3200\u0026oaad=11000\u0026oavd=11000\u0026ocs=700\u0026oewis=1\u0026oputc=1\u0026ofpcc=1\u0026msp=1\u0026odepv=1\u0026id=3e722b54321c6c69\u0026ip=117.213.17.165\u0026initcwndbps=977500\u0026mt=1679566621\u0026oweuc=\u0026pxtags=Cg4KAnR4EggyNDQyODQxNQ\u0026rxtags=Cg4KAnR4EggyNDQyODQxNA%2CCg4KAnR4EggyNDQyODQxNQ%2CCg4KAnR4EggyNDQyODQxNg%2CCg4KAnR4EggyNDQyODQxNw%2CCg4KAnR4EggyNDQyODQxOA%2CCg4KAnR4EggyNDQyODQxOQ%2CCg4KAnR4EggyNDQyODQyMA"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CLoBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28QOnY8ZDD6oq5PqoBGlVVTEZtamRod01HU3V0OG1aMUNxblJqalV3",
                                                    "showActionMenu": false,
                                                    "shortViewCountText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "190 views"
                                                            }
                                                        },
                                                        "simpleText": "190 views"
                                                    },
                                                    "menu": {
                                                        "menuRenderer": {
                                                            "items": [{
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Add to queue"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "ADD_TO_QUEUE_TAIL"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CL4BEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true
                                                                            }
                                                                        },
                                                                        "signalServiceEndpoint": {
                                                                            "signal": "CLIENT_SIGNAL",
                                                                            "actions": [{
                                                                                "clickTrackingParams": "CL4BEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "addToPlaylistCommand": {
                                                                                    "openMiniplayer": true,
                                                                                    "videoId": "PnIrVDIcbGk",
                                                                                    "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                                    "onCreateListCommand": {
                                                                                        "clickTrackingParams": "CL4BEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                        "commandMetadata": {
                                                                                            "webCommandMetadata": {
                                                                                                "sendPost": true,
                                                                                                "apiUrl": "/youtubei/v1/playlist/create"
                                                                                            }
                                                                                        },
                                                                                        "createPlaylistServiceEndpoint": {
                                                                                            "videoIds": ["PnIrVDIcbGk"],
                                                                                            "params": "CAQ%3D"
                                                                                        }
                                                                                    },
                                                                                    "videoIds": ["PnIrVDIcbGk"]
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CL4BEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                                }
                                                            }, {
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Share"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "SHARE"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CLoBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true,
                                                                                "apiUrl": "/youtubei/v1/share/get_share_panel"
                                                                            }
                                                                        },
                                                                        "shareEntityServiceEndpoint": {
                                                                            "serializedShareEntity": "CgtQbklyVkRJY2JHaw%3D%3D",
                                                                            "commands": [{
                                                                                "clickTrackingParams": "CLoBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "openPopupAction": {
                                                                                    "popup": {
                                                                                        "unifiedSharePanelRenderer": {
                                                                                            "trackingParams": "CL0BEI5iIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                            "showLoadingSpinner": true
                                                                                        }
                                                                                    },
                                                                                    "popupType": "DIALOG",
                                                                                    "beReused": true
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CLoBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                                }
                                                            }],
                                                            "trackingParams": "CLoBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                            "accessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Action menu"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "thumbnailOverlays": [{
                                                        "thumbnailOverlayTimeStatusRenderer": {
                                                            "text": {
                                                                "accessibility": {
                                                                    "accessibilityData": {
                                                                        "label": "10 minutes, 53 seconds"
                                                                    }
                                                                },
                                                                "simpleText": "10:53"
                                                            },
                                                            "style": "DEFAULT"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "isToggled": false,
                                                            "untoggledIcon": {
                                                                "iconType": "WATCH_LATER"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "CHECK"
                                                            },
                                                            "untoggledTooltip": "Watch later",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CLwBEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "addedVideoId": "PnIrVDIcbGk",
                                                                        "action": "ACTION_ADD_VIDEO"
                                                                    }]
                                                                }
                                                            },
                                                            "toggledServiceEndpoint": {
                                                                "clickTrackingParams": "CLwBEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "action": "ACTION_REMOVE_VIDEO_BY_VIDEO_ID",
                                                                        "removedVideoId": "PnIrVDIcbGk"
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Watch later"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CLwBEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "untoggledIcon": {
                                                                "iconType": "ADD_TO_QUEUE_TAIL"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "PLAYLIST_ADD_CHECK"
                                                            },
                                                            "untoggledTooltip": "Add to queue",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CLsBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true
                                                                    }
                                                                },
                                                                "signalServiceEndpoint": {
                                                                    "signal": "CLIENT_SIGNAL",
                                                                    "actions": [{
                                                                        "clickTrackingParams": "CLsBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "addToPlaylistCommand": {
                                                                            "openMiniplayer": true,
                                                                            "videoId": "PnIrVDIcbGk",
                                                                            "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                            "onCreateListCommand": {
                                                                                "clickTrackingParams": "CLsBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "commandMetadata": {
                                                                                    "webCommandMetadata": {
                                                                                        "sendPost": true,
                                                                                        "apiUrl": "/youtubei/v1/playlist/create"
                                                                                    }
                                                                                },
                                                                                "createPlaylistServiceEndpoint": {
                                                                                    "videoIds": ["PnIrVDIcbGk"],
                                                                                    "params": "CAQ%3D"
                                                                                }
                                                                            },
                                                                            "videoIds": ["PnIrVDIcbGk"]
                                                                        }
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Add to queue"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CLsBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayNowPlayingRenderer": {
                                                            "text": {
                                                                "runs": [{
                                                                    "text": "Now playing"
                                                                }]
                                                            }
                                                        }
                                                    }],
                                                    "richThumbnail": {
                                                        "movingThumbnailRenderer": {
                                                            "movingThumbnailDetails": {
                                                                "thumbnails": [{
                                                                    "url": "https://i.ytimg.com/an_webp/PnIrVDIcbGk/mqdefault_6s.webp?du=3000\u0026sqp=CNnJ8KAG\u0026rs=AOn4CLAON5P7fMyQSSYJvzhHQQlO4GfqNA",
                                                                    "width": 320,
                                                                    "height": 180
                                                                }],
                                                                "logAsMovingThumbnail": true
                                                            },
                                                            "enableHoveredLogging": true,
                                                            "enableOverlay": true
                                                        }
                                                    }
                                                }
                                            },
                                            "trackingParams": "CLkBEJmNBRgDIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                        }
                                    }, {
                                        "richItemRenderer": {
                                            "content": {
                                                "videoRenderer": {
                                                    "videoId": "xcxB8V26Py4",
                                                    "thumbnail": {
                                                        "thumbnails": [{
                                                            "url": "https://i.ytimg.com/vi/xcxB8V26Py4/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLCTZj_8MNjlg4017WL41JUatuqoHA",
                                                            "width": 168,
                                                            "height": 94
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/xcxB8V26Py4/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLCgK1e05d7O1VfBbZg3lWo5Z_kOEQ",
                                                            "width": 196,
                                                            "height": 110
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/xcxB8V26Py4/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLD1T_b63dQ7N0ez_jEfsWAYl0JIQQ",
                                                            "width": 246,
                                                            "height": 138
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/xcxB8V26Py4/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLCe0lh4DBmvukE6VGXQBREHnB5C-w",
                                                            "width": 336,
                                                            "height": 188
                                                        }]
                                                    },
                                                    "title": {
                                                        "runs": [{
                                                            "text": "Git patch是什么？ | Git patch \u0026 apply | git还能发邮件呢！"
                                                        }],
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "Git patch是什么？ | Git patch \u0026 apply | git还能发邮件呢！ by 麦兜搞IT 3 weeks ago 10 minutes, 30 seconds 56 views"
                                                            }
                                                        }
                                                    },
                                                    "descriptionSnippet": {
                                                        "runs": [{
                                                            "text": "https://git-scm.com/docs/git-format-patch\nhttps://git-scm.com/docs/git-send-email\nhttps://lkml.org/\n===============================\n【我的一些在线课程】\n\nUdemy：https://github.com/udemy-..."
                                                        }]
                                                    },
                                                    "publishedTimeText": {
                                                        "simpleText": "3 weeks ago"
                                                    },
                                                    "lengthText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "10 minutes, 30 seconds"
                                                            }
                                                        },
                                                        "simpleText": "10:30"
                                                    },
                                                    "viewCountText": {
                                                        "simpleText": "56 views"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CLQBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28WhhVQ21qZGh3TUdTdXQ4bVoxQ3FuUmpqVXeaAQYQ8jgY4AeqARpVVUxGbWpkaHdNR1N1dDhtWjFDcW5SampVdw==",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "url": "/watch?v=xcxB8V26Py4",
                                                                "webPageType": "WEB_PAGE_TYPE_WATCH",
                                                                "rootVe": 3832
                                                            }
                                                        },
                                                        "watchEndpoint": {
                                                            "videoId": "xcxB8V26Py4",
                                                            "watchEndpointSupportedOnesieConfig": {
                                                                "html5PlaybackOnesieConfig": {
                                                                    "commonConfig": {
                                                                        "url": "https://rr7---sn-cnoa-jv3s.googlevideo.com/initplayback?source=youtube\u0026oeis=1\u0026c=WEB\u0026oad=3200\u0026ovd=3200\u0026oaad=11000\u0026oavd=11000\u0026ocs=700\u0026oewis=1\u0026oputc=1\u0026ofpcc=1\u0026msp=1\u0026odepv=1\u0026id=c5cc41f15dba3f2e\u0026ip=117.213.17.165\u0026initcwndbps=977500\u0026mt=1679566621\u0026oweuc=\u0026pxtags=Cg4KAnR4EggyNDQyODQxNQ\u0026rxtags=Cg4KAnR4EggyNDQyODQxNA%2CCg4KAnR4EggyNDQyODQxNQ%2CCg4KAnR4EggyNDQyODQxNg%2CCg4KAnR4EggyNDQyODQxNw%2CCg4KAnR4EggyNDQyODQxOA%2CCg4KAnR4EggyNDQyODQxOQ%2CCg4KAnR4EggyNDQyODQyMA"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CLQBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28QK7-6O2VvpDmxQGqARpVVUxGbWpkaHdNR1N1dDhtWjFDcW5SampVdw==",
                                                    "showActionMenu": false,
                                                    "shortViewCountText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "56 views"
                                                            }
                                                        },
                                                        "simpleText": "56 views"
                                                    },
                                                    "menu": {
                                                        "menuRenderer": {
                                                            "items": [{
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Add to queue"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "ADD_TO_QUEUE_TAIL"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CLgBEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true
                                                                            }
                                                                        },
                                                                        "signalServiceEndpoint": {
                                                                            "signal": "CLIENT_SIGNAL",
                                                                            "actions": [{
                                                                                "clickTrackingParams": "CLgBEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "addToPlaylistCommand": {
                                                                                    "openMiniplayer": true,
                                                                                    "videoId": "xcxB8V26Py4",
                                                                                    "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                                    "onCreateListCommand": {
                                                                                        "clickTrackingParams": "CLgBEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                        "commandMetadata": {
                                                                                            "webCommandMetadata": {
                                                                                                "sendPost": true,
                                                                                                "apiUrl": "/youtubei/v1/playlist/create"
                                                                                            }
                                                                                        },
                                                                                        "createPlaylistServiceEndpoint": {
                                                                                            "videoIds": ["xcxB8V26Py4"],
                                                                                            "params": "CAQ%3D"
                                                                                        }
                                                                                    },
                                                                                    "videoIds": ["xcxB8V26Py4"]
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CLgBEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                                }
                                                            }, {
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Share"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "SHARE"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CLQBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true,
                                                                                "apiUrl": "/youtubei/v1/share/get_share_panel"
                                                                            }
                                                                        },
                                                                        "shareEntityServiceEndpoint": {
                                                                            "serializedShareEntity": "Cgt4Y3hCOFYyNlB5NA%3D%3D",
                                                                            "commands": [{
                                                                                "clickTrackingParams": "CLQBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "openPopupAction": {
                                                                                    "popup": {
                                                                                        "unifiedSharePanelRenderer": {
                                                                                            "trackingParams": "CLcBEI5iIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                            "showLoadingSpinner": true
                                                                                        }
                                                                                    },
                                                                                    "popupType": "DIALOG",
                                                                                    "beReused": true
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CLQBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                                }
                                                            }],
                                                            "trackingParams": "CLQBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                            "accessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Action menu"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "thumbnailOverlays": [{
                                                        "thumbnailOverlayTimeStatusRenderer": {
                                                            "text": {
                                                                "accessibility": {
                                                                    "accessibilityData": {
                                                                        "label": "10 minutes, 30 seconds"
                                                                    }
                                                                },
                                                                "simpleText": "10:30"
                                                            },
                                                            "style": "DEFAULT"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "isToggled": false,
                                                            "untoggledIcon": {
                                                                "iconType": "WATCH_LATER"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "CHECK"
                                                            },
                                                            "untoggledTooltip": "Watch later",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CLYBEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "addedVideoId": "xcxB8V26Py4",
                                                                        "action": "ACTION_ADD_VIDEO"
                                                                    }]
                                                                }
                                                            },
                                                            "toggledServiceEndpoint": {
                                                                "clickTrackingParams": "CLYBEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "action": "ACTION_REMOVE_VIDEO_BY_VIDEO_ID",
                                                                        "removedVideoId": "xcxB8V26Py4"
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Watch later"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CLYBEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "untoggledIcon": {
                                                                "iconType": "ADD_TO_QUEUE_TAIL"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "PLAYLIST_ADD_CHECK"
                                                            },
                                                            "untoggledTooltip": "Add to queue",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CLUBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true
                                                                    }
                                                                },
                                                                "signalServiceEndpoint": {
                                                                    "signal": "CLIENT_SIGNAL",
                                                                    "actions": [{
                                                                        "clickTrackingParams": "CLUBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "addToPlaylistCommand": {
                                                                            "openMiniplayer": true,
                                                                            "videoId": "xcxB8V26Py4",
                                                                            "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                            "onCreateListCommand": {
                                                                                "clickTrackingParams": "CLUBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "commandMetadata": {
                                                                                    "webCommandMetadata": {
                                                                                        "sendPost": true,
                                                                                        "apiUrl": "/youtubei/v1/playlist/create"
                                                                                    }
                                                                                },
                                                                                "createPlaylistServiceEndpoint": {
                                                                                    "videoIds": ["xcxB8V26Py4"],
                                                                                    "params": "CAQ%3D"
                                                                                }
                                                                            },
                                                                            "videoIds": ["xcxB8V26Py4"]
                                                                        }
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Add to queue"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CLUBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayNowPlayingRenderer": {
                                                            "text": {
                                                                "runs": [{
                                                                    "text": "Now playing"
                                                                }]
                                                            }
                                                        }
                                                    }],
                                                    "richThumbnail": {
                                                        "movingThumbnailRenderer": {
                                                            "movingThumbnailDetails": {
                                                                "thumbnails": [{
                                                                    "url": "https://i.ytimg.com/an_webp/xcxB8V26Py4/mqdefault_6s.webp?du=3000\u0026sqp=CLDO8KAG\u0026rs=AOn4CLD9ZEE8i2KdCDAs1VRZDVM_vzxKZQ",
                                                                    "width": 320,
                                                                    "height": 180
                                                                }],
                                                                "logAsMovingThumbnail": true
                                                            },
                                                            "enableHoveredLogging": true,
                                                            "enableOverlay": true
                                                        }
                                                    }
                                                }
                                            },
                                            "trackingParams": "CLMBEJmNBRgEIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                        }
                                    }, {
                                        "richItemRenderer": {
                                            "content": {
                                                "videoRenderer": {
                                                    "videoId": "bM4k9nuhv54",
                                                    "thumbnail": {
                                                        "thumbnails": [{
                                                            "url": "https://i.ytimg.com/vi/bM4k9nuhv54/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLCVQp1sUaAjXhfH0YDXpBMbmv3eAQ",
                                                            "width": 168,
                                                            "height": 94
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/bM4k9nuhv54/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLBeKRf-aUbQpCq26UrbXP85_9_YqA",
                                                            "width": 196,
                                                            "height": 110
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/bM4k9nuhv54/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLA8zn_q8qZ62oSWPq0ASZpQ-LA7Dg",
                                                            "width": 246,
                                                            "height": 138
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/bM4k9nuhv54/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLDiEGzGbHCjOoA7jZhaEQuTBE3stA",
                                                            "width": 336,
                                                            "height": 188
                                                        }]
                                                    },
                                                    "title": {
                                                        "runs": [{
                                                            "text": "什么是 git cherry pick？| 使用场景是什么？"
                                                        }],
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "什么是 git cherry pick？| 使用场景是什么？ by 麦兜搞IT 3 weeks ago 9 minutes, 24 seconds 102 views"
                                                            }
                                                        }
                                                    },
                                                    "descriptionSnippet": {
                                                        "runs": [{
                                                            "text": "===============================\n【我的一些在线课程】\n\nUdemy：https://github.com/udemy-course\nHiskio：https://hiskio.com/search?word=peng%20xiao\u0026sort=attendance\u0026page=1\n\n【联系方式..."
                                                        }]
                                                    },
                                                    "publishedTimeText": {
                                                        "simpleText": "3 weeks ago"
                                                    },
                                                    "lengthText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "9 minutes, 24 seconds"
                                                            }
                                                        },
                                                        "simpleText": "9:24"
                                                    },
                                                    "viewCountText": {
                                                        "simpleText": "102 views"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CK4BENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28WhhVQ21qZGh3TUdTdXQ4bVoxQ3FuUmpqVXeaAQYQ8jgY4AeqARpVVUxGbWpkaHdNR1N1dDhtWjFDcW5SampVdw==",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "url": "/watch?v=bM4k9nuhv54",
                                                                "webPageType": "WEB_PAGE_TYPE_WATCH",
                                                                "rootVe": 3832
                                                            }
                                                        },
                                                        "watchEndpoint": {
                                                            "videoId": "bM4k9nuhv54",
                                                            "watchEndpointSupportedOnesieConfig": {
                                                                "html5PlaybackOnesieConfig": {
                                                                    "commonConfig": {
                                                                        "url": "https://rr4---sn-cnoa-jv3s.googlevideo.com/initplayback?source=youtube\u0026oeis=1\u0026c=WEB\u0026oad=3200\u0026ovd=3200\u0026oaad=11000\u0026oavd=11000\u0026ocs=700\u0026oewis=1\u0026oputc=1\u0026ofpcc=1\u0026msp=1\u0026odepv=1\u0026id=6cce24f67ba1bf9e\u0026ip=117.213.17.165\u0026initcwndbps=977500\u0026mt=1679566621\u0026oweuc=\u0026pxtags=Cg4KAnR4EggyNDQyODQxNQ\u0026rxtags=Cg4KAnR4EggyNDQyODQxNA%2CCg4KAnR4EggyNDQyODQxNQ%2CCg4KAnR4EggyNDQyODQxNg%2CCg4KAnR4EggyNDQyODQxNw%2CCg4KAnR4EggyNDQyODQxOA%2CCg4KAnR4EggyNDQyODQxOQ%2CCg4KAnR4EggyNDQyODQyMA"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CK4BENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28QJ7_ht3nnonnbKoBGlVVTEZtamRod01HU3V0OG1aMUNxblJqalV3",
                                                    "showActionMenu": false,
                                                    "shortViewCountText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "102 views"
                                                            }
                                                        },
                                                        "simpleText": "102 views"
                                                    },
                                                    "menu": {
                                                        "menuRenderer": {
                                                            "items": [{
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Add to queue"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "ADD_TO_QUEUE_TAIL"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CLIBEP6YBBgFIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true
                                                                            }
                                                                        },
                                                                        "signalServiceEndpoint": {
                                                                            "signal": "CLIENT_SIGNAL",
                                                                            "actions": [{
                                                                                "clickTrackingParams": "CLIBEP6YBBgFIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "addToPlaylistCommand": {
                                                                                    "openMiniplayer": true,
                                                                                    "videoId": "bM4k9nuhv54",
                                                                                    "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                                    "onCreateListCommand": {
                                                                                        "clickTrackingParams": "CLIBEP6YBBgFIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                        "commandMetadata": {
                                                                                            "webCommandMetadata": {
                                                                                                "sendPost": true,
                                                                                                "apiUrl": "/youtubei/v1/playlist/create"
                                                                                            }
                                                                                        },
                                                                                        "createPlaylistServiceEndpoint": {
                                                                                            "videoIds": ["bM4k9nuhv54"],
                                                                                            "params": "CAQ%3D"
                                                                                        }
                                                                                    },
                                                                                    "videoIds": ["bM4k9nuhv54"]
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CLIBEP6YBBgFIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                                }
                                                            }, {
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Share"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "SHARE"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CK4BENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true,
                                                                                "apiUrl": "/youtubei/v1/share/get_share_panel"
                                                                            }
                                                                        },
                                                                        "shareEntityServiceEndpoint": {
                                                                            "serializedShareEntity": "CgtiTTRrOW51aHY1NA%3D%3D",
                                                                            "commands": [{
                                                                                "clickTrackingParams": "CK4BENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "openPopupAction": {
                                                                                    "popup": {
                                                                                        "unifiedSharePanelRenderer": {
                                                                                            "trackingParams": "CLEBEI5iIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                            "showLoadingSpinner": true
                                                                                        }
                                                                                    },
                                                                                    "popupType": "DIALOG",
                                                                                    "beReused": true
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CK4BENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                                }
                                                            }],
                                                            "trackingParams": "CK4BENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                            "accessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Action menu"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "thumbnailOverlays": [{
                                                        "thumbnailOverlayTimeStatusRenderer": {
                                                            "text": {
                                                                "accessibility": {
                                                                    "accessibilityData": {
                                                                        "label": "9 minutes, 24 seconds"
                                                                    }
                                                                },
                                                                "simpleText": "9:24"
                                                            },
                                                            "style": "DEFAULT"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "isToggled": false,
                                                            "untoggledIcon": {
                                                                "iconType": "WATCH_LATER"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "CHECK"
                                                            },
                                                            "untoggledTooltip": "Watch later",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CLABEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "addedVideoId": "bM4k9nuhv54",
                                                                        "action": "ACTION_ADD_VIDEO"
                                                                    }]
                                                                }
                                                            },
                                                            "toggledServiceEndpoint": {
                                                                "clickTrackingParams": "CLABEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "action": "ACTION_REMOVE_VIDEO_BY_VIDEO_ID",
                                                                        "removedVideoId": "bM4k9nuhv54"
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Watch later"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CLABEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "untoggledIcon": {
                                                                "iconType": "ADD_TO_QUEUE_TAIL"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "PLAYLIST_ADD_CHECK"
                                                            },
                                                            "untoggledTooltip": "Add to queue",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CK8BEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true
                                                                    }
                                                                },
                                                                "signalServiceEndpoint": {
                                                                    "signal": "CLIENT_SIGNAL",
                                                                    "actions": [{
                                                                        "clickTrackingParams": "CK8BEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "addToPlaylistCommand": {
                                                                            "openMiniplayer": true,
                                                                            "videoId": "bM4k9nuhv54",
                                                                            "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                            "onCreateListCommand": {
                                                                                "clickTrackingParams": "CK8BEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "commandMetadata": {
                                                                                    "webCommandMetadata": {
                                                                                        "sendPost": true,
                                                                                        "apiUrl": "/youtubei/v1/playlist/create"
                                                                                    }
                                                                                },
                                                                                "createPlaylistServiceEndpoint": {
                                                                                    "videoIds": ["bM4k9nuhv54"],
                                                                                    "params": "CAQ%3D"
                                                                                }
                                                                            },
                                                                            "videoIds": ["bM4k9nuhv54"]
                                                                        }
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Add to queue"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CK8BEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayNowPlayingRenderer": {
                                                            "text": {
                                                                "runs": [{
                                                                    "text": "Now playing"
                                                                }]
                                                            }
                                                        }
                                                    }]
                                                }
                                            },
                                            "trackingParams": "CK0BEJmNBRgFIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                        }
                                    }, {
                                        "richItemRenderer": {
                                            "content": {
                                                "videoRenderer": {
                                                    "videoId": "h07pWeUiSv0",
                                                    "thumbnail": {
                                                        "thumbnails": [{
                                                            "url": "https://i.ytimg.com/vi/h07pWeUiSv0/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLC278XL4_pWbonLTgUbH2oPKzlxZg",
                                                            "width": 168,
                                                            "height": 94
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/h07pWeUiSv0/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLD139Wgm6saOFcFynpJg4c82QrVsg",
                                                            "width": 196,
                                                            "height": 110
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/h07pWeUiSv0/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLDeHfYQOCg9RAYqFkO2gRyowcNxGQ",
                                                            "width": 246,
                                                            "height": 138
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/h07pWeUiSv0/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLBGM5-D89PN15yemkNHGAv-Ioou8A",
                                                            "width": 336,
                                                            "height": 188
                                                        }]
                                                    },
                                                    "title": {
                                                        "runs": [{
                                                            "text": "8. 想继续学习GitHub Actions么? 不要错过这个视频！| 内含广告"
                                                        }],
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "8. 想继续学习GitHub Actions么? 不要错过这个视频！| 内含广告 by 麦兜搞IT 1 month ago 3 minutes, 10 seconds 43 views"
                                                            }
                                                        }
                                                    },
                                                    "descriptionSnippet": {
                                                        "runs": [{
                                                            "text": "完整课程 《GitHub Actions入门与进阶》购买方式：\n- Udemy： https://www.udemy.com/course/github-actions-zh/?couponCode=2023-FEB22-4947790\n- 51CTO:  https://edu.51cto.com/course/33216..."
                                                        }]
                                                    },
                                                    "publishedTimeText": {
                                                        "simpleText": "1 month ago"
                                                    },
                                                    "lengthText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "3 minutes, 10 seconds"
                                                            }
                                                        },
                                                        "simpleText": "3:10"
                                                    },
                                                    "viewCountText": {
                                                        "simpleText": "43 views"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CKgBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28WhhVQ21qZGh3TUdTdXQ4bVoxQ3FuUmpqVXeaAQYQ8jgY4AeqARpVVUxGbWpkaHdNR1N1dDhtWjFDcW5SampVdw==",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "url": "/watch?v=h07pWeUiSv0",
                                                                "webPageType": "WEB_PAGE_TYPE_WATCH",
                                                                "rootVe": 3832
                                                            }
                                                        },
                                                        "watchEndpoint": {
                                                            "videoId": "h07pWeUiSv0",
                                                            "watchEndpointSupportedOnesieConfig": {
                                                                "html5PlaybackOnesieConfig": {
                                                                    "commonConfig": {
                                                                        "url": "https://rr6---sn-cnoa-jv3s.googlevideo.com/initplayback?source=youtube\u0026oeis=1\u0026c=WEB\u0026oad=3200\u0026ovd=3200\u0026oaad=11000\u0026oavd=11000\u0026ocs=700\u0026oewis=1\u0026oputc=1\u0026ofpcc=1\u0026msp=1\u0026odepv=1\u0026id=874ee959e5224afd\u0026ip=117.213.17.165\u0026initcwndbps=977500\u0026mt=1679566621\u0026oweuc=\u0026pxtags=Cg4KAnR4EggyNDQyODQxNQ\u0026rxtags=Cg4KAnR4EggyNDQyODQxNA%2CCg4KAnR4EggyNDQyODQxNQ%2CCg4KAnR4EggyNDQyODQxNg%2CCg4KAnR4EggyNDQyODQxNw%2CCg4KAnR4EggyNDQyODQxOA%2CCg4KAnR4EggyNDQyODQxOQ%2CCg4KAnR4EggyNDQyODQyMA"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CKgBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28QP2Viameq7qnhwGqARpVVUxGbWpkaHdNR1N1dDhtWjFDcW5SampVdw==",
                                                    "showActionMenu": false,
                                                    "shortViewCountText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "43 views"
                                                            }
                                                        },
                                                        "simpleText": "43 views"
                                                    },
                                                    "menu": {
                                                        "menuRenderer": {
                                                            "items": [{
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Add to queue"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "ADD_TO_QUEUE_TAIL"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CKwBEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true
                                                                            }
                                                                        },
                                                                        "signalServiceEndpoint": {
                                                                            "signal": "CLIENT_SIGNAL",
                                                                            "actions": [{
                                                                                "clickTrackingParams": "CKwBEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "addToPlaylistCommand": {
                                                                                    "openMiniplayer": true,
                                                                                    "videoId": "h07pWeUiSv0",
                                                                                    "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                                    "onCreateListCommand": {
                                                                                        "clickTrackingParams": "CKwBEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                        "commandMetadata": {
                                                                                            "webCommandMetadata": {
                                                                                                "sendPost": true,
                                                                                                "apiUrl": "/youtubei/v1/playlist/create"
                                                                                            }
                                                                                        },
                                                                                        "createPlaylistServiceEndpoint": {
                                                                                            "videoIds": ["h07pWeUiSv0"],
                                                                                            "params": "CAQ%3D"
                                                                                        }
                                                                                    },
                                                                                    "videoIds": ["h07pWeUiSv0"]
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CKwBEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                                }
                                                            }, {
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Share"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "SHARE"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CKgBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true,
                                                                                "apiUrl": "/youtubei/v1/share/get_share_panel"
                                                                            }
                                                                        },
                                                                        "shareEntityServiceEndpoint": {
                                                                            "serializedShareEntity": "CgtoMDdwV2VVaVN2MA%3D%3D",
                                                                            "commands": [{
                                                                                "clickTrackingParams": "CKgBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "openPopupAction": {
                                                                                    "popup": {
                                                                                        "unifiedSharePanelRenderer": {
                                                                                            "trackingParams": "CKsBEI5iIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                            "showLoadingSpinner": true
                                                                                        }
                                                                                    },
                                                                                    "popupType": "DIALOG",
                                                                                    "beReused": true
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CKgBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                                }
                                                            }],
                                                            "trackingParams": "CKgBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                            "accessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Action menu"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "thumbnailOverlays": [{
                                                        "thumbnailOverlayTimeStatusRenderer": {
                                                            "text": {
                                                                "accessibility": {
                                                                    "accessibilityData": {
                                                                        "label": "3 minutes, 10 seconds"
                                                                    }
                                                                },
                                                                "simpleText": "3:10"
                                                            },
                                                            "style": "DEFAULT"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "isToggled": false,
                                                            "untoggledIcon": {
                                                                "iconType": "WATCH_LATER"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "CHECK"
                                                            },
                                                            "untoggledTooltip": "Watch later",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CKoBEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "addedVideoId": "h07pWeUiSv0",
                                                                        "action": "ACTION_ADD_VIDEO"
                                                                    }]
                                                                }
                                                            },
                                                            "toggledServiceEndpoint": {
                                                                "clickTrackingParams": "CKoBEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "action": "ACTION_REMOVE_VIDEO_BY_VIDEO_ID",
                                                                        "removedVideoId": "h07pWeUiSv0"
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Watch later"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CKoBEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "untoggledIcon": {
                                                                "iconType": "ADD_TO_QUEUE_TAIL"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "PLAYLIST_ADD_CHECK"
                                                            },
                                                            "untoggledTooltip": "Add to queue",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CKkBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true
                                                                    }
                                                                },
                                                                "signalServiceEndpoint": {
                                                                    "signal": "CLIENT_SIGNAL",
                                                                    "actions": [{
                                                                        "clickTrackingParams": "CKkBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "addToPlaylistCommand": {
                                                                            "openMiniplayer": true,
                                                                            "videoId": "h07pWeUiSv0",
                                                                            "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                            "onCreateListCommand": {
                                                                                "clickTrackingParams": "CKkBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "commandMetadata": {
                                                                                    "webCommandMetadata": {
                                                                                        "sendPost": true,
                                                                                        "apiUrl": "/youtubei/v1/playlist/create"
                                                                                    }
                                                                                },
                                                                                "createPlaylistServiceEndpoint": {
                                                                                    "videoIds": ["h07pWeUiSv0"],
                                                                                    "params": "CAQ%3D"
                                                                                }
                                                                            },
                                                                            "videoIds": ["h07pWeUiSv0"]
                                                                        }
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Add to queue"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CKkBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayNowPlayingRenderer": {
                                                            "text": {
                                                                "runs": [{
                                                                    "text": "Now playing"
                                                                }]
                                                            }
                                                        }
                                                    }],
                                                    "richThumbnail": {
                                                        "movingThumbnailRenderer": {
                                                            "movingThumbnailDetails": {
                                                                "thumbnails": [{
                                                                    "url": "https://i.ytimg.com/an_webp/h07pWeUiSv0/mqdefault_6s.webp?du=3000\u0026sqp=CKqw8KAG\u0026rs=AOn4CLAptcv-XSwsLgyqA_cPhWoeRXorXg",
                                                                    "width": 320,
                                                                    "height": 180
                                                                }],
                                                                "logAsMovingThumbnail": true
                                                            },
                                                            "enableHoveredLogging": true,
                                                            "enableOverlay": true
                                                        }
                                                    }
                                                }
                                            },
                                            "trackingParams": "CKcBEJmNBRgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                        }
                                    }, {
                                        "richItemRenderer": {
                                            "content": {
                                                "videoRenderer": {
                                                    "videoId": "ZWbHfmuzZ8Q",
                                                    "thumbnail": {
                                                        "thumbnails": [{
                                                            "url": "https://i.ytimg.com/vi/ZWbHfmuzZ8Q/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLBGimckvF-F01aV3pDxTxVrQYCpxg",
                                                            "width": 168,
                                                            "height": 94
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/ZWbHfmuzZ8Q/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLAqp4U46qQHRnw6TKvYiK19DVki3g",
                                                            "width": 196,
                                                            "height": 110
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/ZWbHfmuzZ8Q/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLA0EpfhEtAIeUhMOB4-tte57KQLsA",
                                                            "width": 246,
                                                            "height": 138
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/ZWbHfmuzZ8Q/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLDvvn18TcREA2MJNBzZm-nF24u9mQ",
                                                            "width": 336,
                                                            "height": 188
                                                        }]
                                                    },
                                                    "title": {
                                                        "runs": [{
                                                            "text": "7.GitHub Actions之使用Actions"
                                                        }],
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "7.GitHub Actions之使用Actions by 麦兜搞IT 1 month ago 9 minutes, 37 seconds 33 views"
                                                            }
                                                        }
                                                    },
                                                    "descriptionSnippet": {
                                                        "runs": [{
                                                            "text": "===============================\n【我的一些在线课程】\n\nUdemy：https://github.com/udemy-course\nHiskio：https://hiskio.com/search?word=peng%20xiao\u0026sort=attendance\u0026page=1\n\n【联系方式..."
                                                        }]
                                                    },
                                                    "publishedTimeText": {
                                                        "simpleText": "1 month ago"
                                                    },
                                                    "lengthText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "9 minutes, 37 seconds"
                                                            }
                                                        },
                                                        "simpleText": "9:37"
                                                    },
                                                    "viewCountText": {
                                                        "simpleText": "33 views"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CKIBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28WhhVQ21qZGh3TUdTdXQ4bVoxQ3FuUmpqVXeaAQYQ8jgY4AeqARpVVUxGbWpkaHdNR1N1dDhtWjFDcW5SampVdw==",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "url": "/watch?v=ZWbHfmuzZ8Q",
                                                                "webPageType": "WEB_PAGE_TYPE_WATCH",
                                                                "rootVe": 3832
                                                            }
                                                        },
                                                        "watchEndpoint": {
                                                            "videoId": "ZWbHfmuzZ8Q",
                                                            "watchEndpointSupportedOnesieConfig": {
                                                                "html5PlaybackOnesieConfig": {
                                                                    "commonConfig": {
                                                                        "url": "https://rr2---sn-cnoa-jv3s.googlevideo.com/initplayback?source=youtube\u0026oeis=1\u0026c=WEB\u0026oad=3200\u0026ovd=3200\u0026oaad=11000\u0026oavd=11000\u0026ocs=700\u0026oewis=1\u0026oputc=1\u0026ofpcc=1\u0026msp=1\u0026odepv=1\u0026id=6566c77e6bb367c4\u0026ip=117.213.17.165\u0026initcwndbps=977500\u0026mt=1679566621\u0026oweuc=\u0026pxtags=Cg4KAnR4EggyNDQyODQxNQ\u0026rxtags=Cg4KAnR4EggyNDQyODQxNA%2CCg4KAnR4EggyNDQyODQxNQ%2CCg4KAnR4EggyNDQyODQxNg%2CCg4KAnR4EggyNDQyODQxNw%2CCg4KAnR4EggyNDQyODQxOA%2CCg4KAnR4EggyNDQyODQxOQ%2CCg4KAnR4EggyNDQyODQyMA"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CKIBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28QMTPzd3m77GzZaoBGlVVTEZtamRod01HU3V0OG1aMUNxblJqalV3",
                                                    "showActionMenu": false,
                                                    "shortViewCountText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "33 views"
                                                            }
                                                        },
                                                        "simpleText": "33 views"
                                                    },
                                                    "menu": {
                                                        "menuRenderer": {
                                                            "items": [{
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Add to queue"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "ADD_TO_QUEUE_TAIL"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CKYBEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true
                                                                            }
                                                                        },
                                                                        "signalServiceEndpoint": {
                                                                            "signal": "CLIENT_SIGNAL",
                                                                            "actions": [{
                                                                                "clickTrackingParams": "CKYBEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "addToPlaylistCommand": {
                                                                                    "openMiniplayer": true,
                                                                                    "videoId": "ZWbHfmuzZ8Q",
                                                                                    "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                                    "onCreateListCommand": {
                                                                                        "clickTrackingParams": "CKYBEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                        "commandMetadata": {
                                                                                            "webCommandMetadata": {
                                                                                                "sendPost": true,
                                                                                                "apiUrl": "/youtubei/v1/playlist/create"
                                                                                            }
                                                                                        },
                                                                                        "createPlaylistServiceEndpoint": {
                                                                                            "videoIds": ["ZWbHfmuzZ8Q"],
                                                                                            "params": "CAQ%3D"
                                                                                        }
                                                                                    },
                                                                                    "videoIds": ["ZWbHfmuzZ8Q"]
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CKYBEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                                }
                                                            }, {
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Share"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "SHARE"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CKIBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true,
                                                                                "apiUrl": "/youtubei/v1/share/get_share_panel"
                                                                            }
                                                                        },
                                                                        "shareEntityServiceEndpoint": {
                                                                            "serializedShareEntity": "CgtaV2JIZm11elo4UQ%3D%3D",
                                                                            "commands": [{
                                                                                "clickTrackingParams": "CKIBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "openPopupAction": {
                                                                                    "popup": {
                                                                                        "unifiedSharePanelRenderer": {
                                                                                            "trackingParams": "CKUBEI5iIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                            "showLoadingSpinner": true
                                                                                        }
                                                                                    },
                                                                                    "popupType": "DIALOG",
                                                                                    "beReused": true
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CKIBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                                }
                                                            }],
                                                            "trackingParams": "CKIBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                            "accessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Action menu"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "thumbnailOverlays": [{
                                                        "thumbnailOverlayTimeStatusRenderer": {
                                                            "text": {
                                                                "accessibility": {
                                                                    "accessibilityData": {
                                                                        "label": "9 minutes, 37 seconds"
                                                                    }
                                                                },
                                                                "simpleText": "9:37"
                                                            },
                                                            "style": "DEFAULT"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "isToggled": false,
                                                            "untoggledIcon": {
                                                                "iconType": "WATCH_LATER"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "CHECK"
                                                            },
                                                            "untoggledTooltip": "Watch later",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CKQBEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "addedVideoId": "ZWbHfmuzZ8Q",
                                                                        "action": "ACTION_ADD_VIDEO"
                                                                    }]
                                                                }
                                                            },
                                                            "toggledServiceEndpoint": {
                                                                "clickTrackingParams": "CKQBEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "action": "ACTION_REMOVE_VIDEO_BY_VIDEO_ID",
                                                                        "removedVideoId": "ZWbHfmuzZ8Q"
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Watch later"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CKQBEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "untoggledIcon": {
                                                                "iconType": "ADD_TO_QUEUE_TAIL"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "PLAYLIST_ADD_CHECK"
                                                            },
                                                            "untoggledTooltip": "Add to queue",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CKMBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true
                                                                    }
                                                                },
                                                                "signalServiceEndpoint": {
                                                                    "signal": "CLIENT_SIGNAL",
                                                                    "actions": [{
                                                                        "clickTrackingParams": "CKMBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "addToPlaylistCommand": {
                                                                            "openMiniplayer": true,
                                                                            "videoId": "ZWbHfmuzZ8Q",
                                                                            "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                            "onCreateListCommand": {
                                                                                "clickTrackingParams": "CKMBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "commandMetadata": {
                                                                                    "webCommandMetadata": {
                                                                                        "sendPost": true,
                                                                                        "apiUrl": "/youtubei/v1/playlist/create"
                                                                                    }
                                                                                },
                                                                                "createPlaylistServiceEndpoint": {
                                                                                    "videoIds": ["ZWbHfmuzZ8Q"],
                                                                                    "params": "CAQ%3D"
                                                                                }
                                                                            },
                                                                            "videoIds": ["ZWbHfmuzZ8Q"]
                                                                        }
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Add to queue"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CKMBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayNowPlayingRenderer": {
                                                            "text": {
                                                                "runs": [{
                                                                    "text": "Now playing"
                                                                }]
                                                            }
                                                        }
                                                    }],
                                                    "richThumbnail": {
                                                        "movingThumbnailRenderer": {
                                                            "movingThumbnailDetails": {
                                                                "thumbnails": [{
                                                                    "url": "https://i.ytimg.com/an_webp/ZWbHfmuzZ8Q/mqdefault_6s.webp?du=3000\u0026sqp=CLys8KAG\u0026rs=AOn4CLBulZFYqbqCBYJRG1LihVvluXaK2A",
                                                                    "width": 320,
                                                                    "height": 180
                                                                }],
                                                                "logAsMovingThumbnail": true
                                                            },
                                                            "enableHoveredLogging": true,
                                                            "enableOverlay": true
                                                        }
                                                    }
                                                }
                                            },
                                            "trackingParams": "CKEBEJmNBRgHIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                        }
                                    }, {
                                        "richItemRenderer": {
                                            "content": {
                                                "videoRenderer": {
                                                    "videoId": "n2gS2_8IbGg",
                                                    "thumbnail": {
                                                        "thumbnails": [{
                                                            "url": "https://i.ytimg.com/vi/n2gS2_8IbGg/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLB9MuJyRRZ2n-_xpF8YR11gqJhV9g",
                                                            "width": 168,
                                                            "height": 94
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/n2gS2_8IbGg/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLBnQAd-kWQgQJ-_qm_ePBA2fx75Gg",
                                                            "width": 196,
                                                            "height": 110
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/n2gS2_8IbGg/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLBpVre_82bgtytQgljRMB9si29KjA",
                                                            "width": 246,
                                                            "height": 138
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/n2gS2_8IbGg/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLBARxozC5ZyXPNR82WjE00HuPvqUQ",
                                                            "width": 336,
                                                            "height": 188
                                                        }]
                                                    },
                                                    "title": {
                                                        "runs": [{
                                                            "text": "微软将ChatGPT背后的AI技术整合到必应搜索引擎和Edge浏览器中 | Google危险了？"
                                                        }],
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "微软将ChatGPT背后的AI技术整合到必应搜索引擎和Edge浏览器中 | Google危险了？ by 麦兜搞IT 1 month ago 8 minutes, 20 seconds 471 views"
                                                            }
                                                        }
                                                    },
                                                    "descriptionSnippet": {
                                                        "runs": [{
                                                            "text": "大家赶紧加入bing chat的waitinglist https://www.bing.com/"
                                                        }]
                                                    },
                                                    "publishedTimeText": {
                                                        "simpleText": "1 month ago"
                                                    },
                                                    "lengthText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "8 minutes, 20 seconds"
                                                            }
                                                        },
                                                        "simpleText": "8:20"
                                                    },
                                                    "viewCountText": {
                                                        "simpleText": "471 views"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CJwBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28WhhVQ21qZGh3TUdTdXQ4bVoxQ3FuUmpqVXeaAQYQ8jgY4AeqARpVVUxGbWpkaHdNR1N1dDhtWjFDcW5SampVdw==",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "url": "/watch?v=n2gS2_8IbGg",
                                                                "webPageType": "WEB_PAGE_TYPE_WATCH",
                                                                "rootVe": 3832
                                                            }
                                                        },
                                                        "watchEndpoint": {
                                                            "videoId": "n2gS2_8IbGg",
                                                            "watchEndpointSupportedOnesieConfig": {
                                                                "html5PlaybackOnesieConfig": {
                                                                    "commonConfig": {
                                                                        "url": "https://rr7---sn-cnoa-jv3s.googlevideo.com/initplayback?source=youtube\u0026oeis=1\u0026c=WEB\u0026oad=3200\u0026ovd=3200\u0026oaad=11000\u0026oavd=11000\u0026ocs=700\u0026oewis=1\u0026oputc=1\u0026ofpcc=1\u0026msp=1\u0026odepv=1\u0026id=9f6812dbff086c68\u0026ip=117.213.17.165\u0026initcwndbps=977500\u0026mt=1679566621\u0026oweuc=\u0026pxtags=Cg4KAnR4EggyNDQyODQxNQ\u0026rxtags=Cg4KAnR4EggyNDQyODQxNA%2CCg4KAnR4EggyNDQyODQxNQ%2CCg4KAnR4EggyNDQyODQxNg%2CCg4KAnR4EggyNDQyODQxNw%2CCg4KAnR4EggyNDQyODQxOA%2CCg4KAnR4EggyNDQyODQxOQ%2CCg4KAnR4EggyNDQyODQyMA"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CJwBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28QOjYofi_24S0nwGqARpVVUxGbWpkaHdNR1N1dDhtWjFDcW5SampVdw==",
                                                    "showActionMenu": false,
                                                    "shortViewCountText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "471 views"
                                                            }
                                                        },
                                                        "simpleText": "471 views"
                                                    },
                                                    "menu": {
                                                        "menuRenderer": {
                                                            "items": [{
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Add to queue"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "ADD_TO_QUEUE_TAIL"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CKABEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true
                                                                            }
                                                                        },
                                                                        "signalServiceEndpoint": {
                                                                            "signal": "CLIENT_SIGNAL",
                                                                            "actions": [{
                                                                                "clickTrackingParams": "CKABEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "addToPlaylistCommand": {
                                                                                    "openMiniplayer": true,
                                                                                    "videoId": "n2gS2_8IbGg",
                                                                                    "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                                    "onCreateListCommand": {
                                                                                        "clickTrackingParams": "CKABEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                        "commandMetadata": {
                                                                                            "webCommandMetadata": {
                                                                                                "sendPost": true,
                                                                                                "apiUrl": "/youtubei/v1/playlist/create"
                                                                                            }
                                                                                        },
                                                                                        "createPlaylistServiceEndpoint": {
                                                                                            "videoIds": ["n2gS2_8IbGg"],
                                                                                            "params": "CAQ%3D"
                                                                                        }
                                                                                    },
                                                                                    "videoIds": ["n2gS2_8IbGg"]
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CKABEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                                }
                                                            }, {
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Share"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "SHARE"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CJwBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true,
                                                                                "apiUrl": "/youtubei/v1/share/get_share_panel"
                                                                            }
                                                                        },
                                                                        "shareEntityServiceEndpoint": {
                                                                            "serializedShareEntity": "CgtuMmdTMl84SWJHZw%3D%3D",
                                                                            "commands": [{
                                                                                "clickTrackingParams": "CJwBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "openPopupAction": {
                                                                                    "popup": {
                                                                                        "unifiedSharePanelRenderer": {
                                                                                            "trackingParams": "CJ8BEI5iIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                            "showLoadingSpinner": true
                                                                                        }
                                                                                    },
                                                                                    "popupType": "DIALOG",
                                                                                    "beReused": true
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CJwBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                                }
                                                            }],
                                                            "trackingParams": "CJwBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                            "accessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Action menu"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "thumbnailOverlays": [{
                                                        "thumbnailOverlayTimeStatusRenderer": {
                                                            "text": {
                                                                "accessibility": {
                                                                    "accessibilityData": {
                                                                        "label": "8 minutes, 20 seconds"
                                                                    }
                                                                },
                                                                "simpleText": "8:20"
                                                            },
                                                            "style": "DEFAULT"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "isToggled": false,
                                                            "untoggledIcon": {
                                                                "iconType": "WATCH_LATER"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "CHECK"
                                                            },
                                                            "untoggledTooltip": "Watch later",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CJ4BEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "addedVideoId": "n2gS2_8IbGg",
                                                                        "action": "ACTION_ADD_VIDEO"
                                                                    }]
                                                                }
                                                            },
                                                            "toggledServiceEndpoint": {
                                                                "clickTrackingParams": "CJ4BEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "action": "ACTION_REMOVE_VIDEO_BY_VIDEO_ID",
                                                                        "removedVideoId": "n2gS2_8IbGg"
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Watch later"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CJ4BEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "untoggledIcon": {
                                                                "iconType": "ADD_TO_QUEUE_TAIL"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "PLAYLIST_ADD_CHECK"
                                                            },
                                                            "untoggledTooltip": "Add to queue",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CJ0BEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true
                                                                    }
                                                                },
                                                                "signalServiceEndpoint": {
                                                                    "signal": "CLIENT_SIGNAL",
                                                                    "actions": [{
                                                                        "clickTrackingParams": "CJ0BEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "addToPlaylistCommand": {
                                                                            "openMiniplayer": true,
                                                                            "videoId": "n2gS2_8IbGg",
                                                                            "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                            "onCreateListCommand": {
                                                                                "clickTrackingParams": "CJ0BEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "commandMetadata": {
                                                                                    "webCommandMetadata": {
                                                                                        "sendPost": true,
                                                                                        "apiUrl": "/youtubei/v1/playlist/create"
                                                                                    }
                                                                                },
                                                                                "createPlaylistServiceEndpoint": {
                                                                                    "videoIds": ["n2gS2_8IbGg"],
                                                                                    "params": "CAQ%3D"
                                                                                }
                                                                            },
                                                                            "videoIds": ["n2gS2_8IbGg"]
                                                                        }
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Add to queue"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CJ0BEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayNowPlayingRenderer": {
                                                            "text": {
                                                                "runs": [{
                                                                    "text": "Now playing"
                                                                }]
                                                            }
                                                        }
                                                    }],
                                                    "richThumbnail": {
                                                        "movingThumbnailRenderer": {
                                                            "movingThumbnailDetails": {
                                                                "thumbnails": [{
                                                                    "url": "https://i.ytimg.com/an_webp/n2gS2_8IbGg/mqdefault_6s.webp?du=3000\u0026sqp=CIC-8KAG\u0026rs=AOn4CLCiWphNi89EnOnuMO0VT6y63OkHxA",
                                                                    "width": 320,
                                                                    "height": 180
                                                                }],
                                                                "logAsMovingThumbnail": true
                                                            },
                                                            "enableHoveredLogging": true,
                                                            "enableOverlay": true
                                                        }
                                                    }
                                                }
                                            },
                                            "trackingParams": "CJsBEJmNBRgIIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                        }
                                    }, {
                                        "richItemRenderer": {
                                            "content": {
                                                "videoRenderer": {
                                                    "videoId": "dfM4JBEUSWY",
                                                    "thumbnail": {
                                                        "thumbnails": [{
                                                            "url": "https://i.ytimg.com/vi/dfM4JBEUSWY/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLBzYGYtzglMihzJ_FTgysbQrcH24Q",
                                                            "width": 168,
                                                            "height": 94
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/dfM4JBEUSWY/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLAkAAlsePMxniOjHuq99HGIC_CPKA",
                                                            "width": 196,
                                                            "height": 110
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/dfM4JBEUSWY/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLDM-ywIjuM8EunX90nWpNHIFg-sgA",
                                                            "width": 246,
                                                            "height": 138
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/dfM4JBEUSWY/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLCBUGU66p-9JLH46DcZqNsHvw6IUA",
                                                            "width": 336,
                                                            "height": 188
                                                        }]
                                                    },
                                                    "title": {
                                                        "runs": [{
                                                            "text": "6. GitHub Actions 一起做一个练习"
                                                        }],
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "6. GitHub Actions 一起做一个练习 by 麦兜搞IT 1 month ago 9 minutes, 25 seconds 46 views"
                                                            }
                                                        }
                                                    },
                                                    "descriptionSnippet": {
                                                        "runs": [{
                                                            "text": "===============================\n【我的一些在线课程】\n\nUdemy：https://github.com/udemy-course\nHiskio：https://hiskio.com/search?word=peng%20xiao\u0026sort=attendance\u0026page=1\n\n【联系方式..."
                                                        }]
                                                    },
                                                    "publishedTimeText": {
                                                        "simpleText": "1 month ago"
                                                    },
                                                    "lengthText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "9 minutes, 25 seconds"
                                                            }
                                                        },
                                                        "simpleText": "9:25"
                                                    },
                                                    "viewCountText": {
                                                        "simpleText": "46 views"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CJYBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28WhhVQ21qZGh3TUdTdXQ4bVoxQ3FuUmpqVXeaAQYQ8jgY4AeqARpVVUxGbWpkaHdNR1N1dDhtWjFDcW5SampVdw==",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "url": "/watch?v=dfM4JBEUSWY",
                                                                "webPageType": "WEB_PAGE_TYPE_WATCH",
                                                                "rootVe": 3832
                                                            }
                                                        },
                                                        "watchEndpoint": {
                                                            "videoId": "dfM4JBEUSWY",
                                                            "watchEndpointSupportedOnesieConfig": {
                                                                "html5PlaybackOnesieConfig": {
                                                                    "commonConfig": {
                                                                        "url": "https://rr7---sn-cnoa-jv3s.googlevideo.com/initplayback?source=youtube\u0026oeis=1\u0026c=WEB\u0026oad=3200\u0026ovd=3200\u0026oaad=11000\u0026oavd=11000\u0026ocs=700\u0026oewis=1\u0026oputc=1\u0026ofpcc=1\u0026msp=1\u0026odepv=1\u0026id=75f3382411144966\u0026ip=117.213.17.165\u0026initcwndbps=977500\u0026mt=1679566621\u0026oweuc=\u0026pxtags=Cg4KAnR4EggyNDQyODQxNQ\u0026rxtags=Cg4KAnR4EggyNDQyODQxNA%2CCg4KAnR4EggyNDQyODQxNQ%2CCg4KAnR4EggyNDQyODQxNg%2CCg4KAnR4EggyNDQyODQxNw%2CCg4KAnR4EggyNDQyODQxOA%2CCg4KAnR4EggyNDQyODQxOQ%2CCg4KAnR4EggyNDQyODQyMA"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CJYBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28QOaS0YjBhM75daoBGlVVTEZtamRod01HU3V0OG1aMUNxblJqalV3",
                                                    "showActionMenu": false,
                                                    "shortViewCountText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "46 views"
                                                            }
                                                        },
                                                        "simpleText": "46 views"
                                                    },
                                                    "menu": {
                                                        "menuRenderer": {
                                                            "items": [{
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Add to queue"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "ADD_TO_QUEUE_TAIL"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CJoBEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true
                                                                            }
                                                                        },
                                                                        "signalServiceEndpoint": {
                                                                            "signal": "CLIENT_SIGNAL",
                                                                            "actions": [{
                                                                                "clickTrackingParams": "CJoBEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "addToPlaylistCommand": {
                                                                                    "openMiniplayer": true,
                                                                                    "videoId": "dfM4JBEUSWY",
                                                                                    "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                                    "onCreateListCommand": {
                                                                                        "clickTrackingParams": "CJoBEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                        "commandMetadata": {
                                                                                            "webCommandMetadata": {
                                                                                                "sendPost": true,
                                                                                                "apiUrl": "/youtubei/v1/playlist/create"
                                                                                            }
                                                                                        },
                                                                                        "createPlaylistServiceEndpoint": {
                                                                                            "videoIds": ["dfM4JBEUSWY"],
                                                                                            "params": "CAQ%3D"
                                                                                        }
                                                                                    },
                                                                                    "videoIds": ["dfM4JBEUSWY"]
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CJoBEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                                }
                                                            }, {
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Share"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "SHARE"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CJYBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true,
                                                                                "apiUrl": "/youtubei/v1/share/get_share_panel"
                                                                            }
                                                                        },
                                                                        "shareEntityServiceEndpoint": {
                                                                            "serializedShareEntity": "CgtkZk00SkJFVVNXWQ%3D%3D",
                                                                            "commands": [{
                                                                                "clickTrackingParams": "CJYBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "openPopupAction": {
                                                                                    "popup": {
                                                                                        "unifiedSharePanelRenderer": {
                                                                                            "trackingParams": "CJkBEI5iIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                            "showLoadingSpinner": true
                                                                                        }
                                                                                    },
                                                                                    "popupType": "DIALOG",
                                                                                    "beReused": true
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CJYBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                                }
                                                            }],
                                                            "trackingParams": "CJYBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                            "accessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Action menu"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "thumbnailOverlays": [{
                                                        "thumbnailOverlayTimeStatusRenderer": {
                                                            "text": {
                                                                "accessibility": {
                                                                    "accessibilityData": {
                                                                        "label": "9 minutes, 25 seconds"
                                                                    }
                                                                },
                                                                "simpleText": "9:25"
                                                            },
                                                            "style": "DEFAULT"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "isToggled": false,
                                                            "untoggledIcon": {
                                                                "iconType": "WATCH_LATER"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "CHECK"
                                                            },
                                                            "untoggledTooltip": "Watch later",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CJgBEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "addedVideoId": "dfM4JBEUSWY",
                                                                        "action": "ACTION_ADD_VIDEO"
                                                                    }]
                                                                }
                                                            },
                                                            "toggledServiceEndpoint": {
                                                                "clickTrackingParams": "CJgBEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "action": "ACTION_REMOVE_VIDEO_BY_VIDEO_ID",
                                                                        "removedVideoId": "dfM4JBEUSWY"
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Watch later"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CJgBEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "untoggledIcon": {
                                                                "iconType": "ADD_TO_QUEUE_TAIL"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "PLAYLIST_ADD_CHECK"
                                                            },
                                                            "untoggledTooltip": "Add to queue",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CJcBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true
                                                                    }
                                                                },
                                                                "signalServiceEndpoint": {
                                                                    "signal": "CLIENT_SIGNAL",
                                                                    "actions": [{
                                                                        "clickTrackingParams": "CJcBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "addToPlaylistCommand": {
                                                                            "openMiniplayer": true,
                                                                            "videoId": "dfM4JBEUSWY",
                                                                            "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                            "onCreateListCommand": {
                                                                                "clickTrackingParams": "CJcBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "commandMetadata": {
                                                                                    "webCommandMetadata": {
                                                                                        "sendPost": true,
                                                                                        "apiUrl": "/youtubei/v1/playlist/create"
                                                                                    }
                                                                                },
                                                                                "createPlaylistServiceEndpoint": {
                                                                                    "videoIds": ["dfM4JBEUSWY"],
                                                                                    "params": "CAQ%3D"
                                                                                }
                                                                            },
                                                                            "videoIds": ["dfM4JBEUSWY"]
                                                                        }
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Add to queue"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CJcBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayNowPlayingRenderer": {
                                                            "text": {
                                                                "runs": [{
                                                                    "text": "Now playing"
                                                                }]
                                                            }
                                                        }
                                                    }],
                                                    "richThumbnail": {
                                                        "movingThumbnailRenderer": {
                                                            "movingThumbnailDetails": {
                                                                "thumbnails": [{
                                                                    "url": "https://i.ytimg.com/an_webp/dfM4JBEUSWY/mqdefault_6s.webp?du=3000\u0026sqp=CNaa8KAG\u0026rs=AOn4CLCif5dmTvRCkwTjC8dgQfwU3W4BYw",
                                                                    "width": 320,
                                                                    "height": 180
                                                                }],
                                                                "logAsMovingThumbnail": true
                                                            },
                                                            "enableHoveredLogging": true,
                                                            "enableOverlay": true
                                                        }
                                                    }
                                                }
                                            },
                                            "trackingParams": "CJUBEJmNBRgJIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                        }
                                    }, {
                                        "richItemRenderer": {
                                            "content": {
                                                "videoRenderer": {
                                                    "videoId": "0I8JVySEGDk",
                                                    "thumbnail": {
                                                        "thumbnails": [{
                                                            "url": "https://i.ytimg.com/vi/0I8JVySEGDk/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLDfXEKQUHF0aQcxmKo-L6fX7-lnug",
                                                            "width": 168,
                                                            "height": 94
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/0I8JVySEGDk/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLCCuL4nCRw8wUXQ8MTLr7f9pXXagw",
                                                            "width": 196,
                                                            "height": 110
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/0I8JVySEGDk/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLDJKBYkQsPguPw5gbGfde8ie5T97A",
                                                            "width": 246,
                                                            "height": 138
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/0I8JVySEGDk/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLDm0wni5fZsu7DQuNVLPxNYSs1HOw",
                                                            "width": 336,
                                                            "height": 188
                                                        }]
                                                    },
                                                    "title": {
                                                        "runs": [{
                                                            "text": "5. GitHub Action里的Jobs的并行和串行"
                                                        }],
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "5. GitHub Action里的Jobs的并行和串行 by 麦兜搞IT 1 month ago 5 minutes, 22 seconds 42 views"
                                                            }
                                                        }
                                                    },
                                                    "descriptionSnippet": {
                                                        "runs": [{
                                                            "text": "===============================\n【我的一些在线课程】\n\nUdemy：https://github.com/udemy-course\nHiskio：https://hiskio.com/search?word=peng%20xiao\u0026sort=attendance\u0026page=1\n\n【联系方式..."
                                                        }]
                                                    },
                                                    "publishedTimeText": {
                                                        "simpleText": "1 month ago"
                                                    },
                                                    "lengthText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "5 minutes, 22 seconds"
                                                            }
                                                        },
                                                        "simpleText": "5:22"
                                                    },
                                                    "viewCountText": {
                                                        "simpleText": "42 views"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CJABENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28WhhVQ21qZGh3TUdTdXQ4bVoxQ3FuUmpqVXeaAQYQ8jgY4AeqARpVVUxGbWpkaHdNR1N1dDhtWjFDcW5SampVdw==",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "url": "/watch?v=0I8JVySEGDk",
                                                                "webPageType": "WEB_PAGE_TYPE_WATCH",
                                                                "rootVe": 3832
                                                            }
                                                        },
                                                        "watchEndpoint": {
                                                            "videoId": "0I8JVySEGDk",
                                                            "watchEndpointSupportedOnesieConfig": {
                                                                "html5PlaybackOnesieConfig": {
                                                                    "commonConfig": {
                                                                        "url": "https://rr5---sn-cnoa-jv3s.googlevideo.com/initplayback?source=youtube\u0026oeis=1\u0026c=WEB\u0026oad=3200\u0026ovd=3200\u0026oaad=11000\u0026oavd=11000\u0026ocs=700\u0026oewis=1\u0026oputc=1\u0026ofpcc=1\u0026msp=1\u0026odepv=1\u0026id=d08f095724841839\u0026ip=117.213.17.165\u0026initcwndbps=977500\u0026mt=1679566621\u0026oweuc=\u0026pxtags=Cg4KAnR4EggyNDQyODQxNQ\u0026rxtags=Cg4KAnR4EggyNDQyODQxNA%2CCg4KAnR4EggyNDQyODQxNQ%2CCg4KAnR4EggyNDQyODQxNg%2CCg4KAnR4EggyNDQyODQxNw%2CCg4KAnR4EggyNDQyODQxOA%2CCg4KAnR4EggyNDQyODQxOQ%2CCg4KAnR4EggyNDQyODQyMA"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CJABENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28QLmwkKTyqsLH0AGqARpVVUxGbWpkaHdNR1N1dDhtWjFDcW5SampVdw==",
                                                    "showActionMenu": false,
                                                    "shortViewCountText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "42 views"
                                                            }
                                                        },
                                                        "simpleText": "42 views"
                                                    },
                                                    "menu": {
                                                        "menuRenderer": {
                                                            "items": [{
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Add to queue"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "ADD_TO_QUEUE_TAIL"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CJQBEP6YBBgFIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true
                                                                            }
                                                                        },
                                                                        "signalServiceEndpoint": {
                                                                            "signal": "CLIENT_SIGNAL",
                                                                            "actions": [{
                                                                                "clickTrackingParams": "CJQBEP6YBBgFIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "addToPlaylistCommand": {
                                                                                    "openMiniplayer": true,
                                                                                    "videoId": "0I8JVySEGDk",
                                                                                    "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                                    "onCreateListCommand": {
                                                                                        "clickTrackingParams": "CJQBEP6YBBgFIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                        "commandMetadata": {
                                                                                            "webCommandMetadata": {
                                                                                                "sendPost": true,
                                                                                                "apiUrl": "/youtubei/v1/playlist/create"
                                                                                            }
                                                                                        },
                                                                                        "createPlaylistServiceEndpoint": {
                                                                                            "videoIds": ["0I8JVySEGDk"],
                                                                                            "params": "CAQ%3D"
                                                                                        }
                                                                                    },
                                                                                    "videoIds": ["0I8JVySEGDk"]
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CJQBEP6YBBgFIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                                }
                                                            }, {
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Share"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "SHARE"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CJABENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true,
                                                                                "apiUrl": "/youtubei/v1/share/get_share_panel"
                                                                            }
                                                                        },
                                                                        "shareEntityServiceEndpoint": {
                                                                            "serializedShareEntity": "CgswSThKVnlTRUdEaw%3D%3D",
                                                                            "commands": [{
                                                                                "clickTrackingParams": "CJABENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "openPopupAction": {
                                                                                    "popup": {
                                                                                        "unifiedSharePanelRenderer": {
                                                                                            "trackingParams": "CJMBEI5iIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                            "showLoadingSpinner": true
                                                                                        }
                                                                                    },
                                                                                    "popupType": "DIALOG",
                                                                                    "beReused": true
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CJABENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                                }
                                                            }],
                                                            "trackingParams": "CJABENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                            "accessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Action menu"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "thumbnailOverlays": [{
                                                        "thumbnailOverlayTimeStatusRenderer": {
                                                            "text": {
                                                                "accessibility": {
                                                                    "accessibilityData": {
                                                                        "label": "5 minutes, 22 seconds"
                                                                    }
                                                                },
                                                                "simpleText": "5:22"
                                                            },
                                                            "style": "DEFAULT"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "isToggled": false,
                                                            "untoggledIcon": {
                                                                "iconType": "WATCH_LATER"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "CHECK"
                                                            },
                                                            "untoggledTooltip": "Watch later",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CJIBEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "addedVideoId": "0I8JVySEGDk",
                                                                        "action": "ACTION_ADD_VIDEO"
                                                                    }]
                                                                }
                                                            },
                                                            "toggledServiceEndpoint": {
                                                                "clickTrackingParams": "CJIBEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "action": "ACTION_REMOVE_VIDEO_BY_VIDEO_ID",
                                                                        "removedVideoId": "0I8JVySEGDk"
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Watch later"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CJIBEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "untoggledIcon": {
                                                                "iconType": "ADD_TO_QUEUE_TAIL"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "PLAYLIST_ADD_CHECK"
                                                            },
                                                            "untoggledTooltip": "Add to queue",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CJEBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true
                                                                    }
                                                                },
                                                                "signalServiceEndpoint": {
                                                                    "signal": "CLIENT_SIGNAL",
                                                                    "actions": [{
                                                                        "clickTrackingParams": "CJEBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "addToPlaylistCommand": {
                                                                            "openMiniplayer": true,
                                                                            "videoId": "0I8JVySEGDk",
                                                                            "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                            "onCreateListCommand": {
                                                                                "clickTrackingParams": "CJEBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "commandMetadata": {
                                                                                    "webCommandMetadata": {
                                                                                        "sendPost": true,
                                                                                        "apiUrl": "/youtubei/v1/playlist/create"
                                                                                    }
                                                                                },
                                                                                "createPlaylistServiceEndpoint": {
                                                                                    "videoIds": ["0I8JVySEGDk"],
                                                                                    "params": "CAQ%3D"
                                                                                }
                                                                            },
                                                                            "videoIds": ["0I8JVySEGDk"]
                                                                        }
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Add to queue"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CJEBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayNowPlayingRenderer": {
                                                            "text": {
                                                                "runs": [{
                                                                    "text": "Now playing"
                                                                }]
                                                            }
                                                        }
                                                    }]
                                                }
                                            },
                                            "trackingParams": "CI8BEJmNBRgKIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                        }
                                    }, {
                                        "richItemRenderer": {
                                            "content": {
                                                "videoRenderer": {
                                                    "videoId": "3NVEwECuDTM",
                                                    "thumbnail": {
                                                        "thumbnails": [{
                                                            "url": "https://i.ytimg.com/vi/3NVEwECuDTM/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLBkmLFMJDu1wANWUphq8goAGuiT7Q",
                                                            "width": 168,
                                                            "height": 94
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/3NVEwECuDTM/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLA3eyZMfRHyblP7aFVAK-qC1JTGPg",
                                                            "width": 196,
                                                            "height": 110
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/3NVEwECuDTM/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLC6WHv_Ctw4ZFqd4ck0_sSiNGPjhQ",
                                                            "width": 246,
                                                            "height": 138
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/3NVEwECuDTM/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLDOruz2LCEAVo40CJX-6d9l-x8W8g",
                                                            "width": 336,
                                                            "height": 188
                                                        }]
                                                    },
                                                    "title": {
                                                        "runs": [{
                                                            "text": "4. GitHub Action Runners"
                                                        }],
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "4. GitHub Action Runners by 麦兜搞IT 1 month ago 8 minutes, 4 seconds 47 views"
                                                            }
                                                        }
                                                    },
                                                    "descriptionSnippet": {
                                                        "runs": [{
                                                            "text": "===============================\n【我的一些在线课程】\n\nUdemy：https://github.com/udemy-course\nHiskio：https://hiskio.com/search?word=peng%20xiao\u0026sort=attendance\u0026page=1\n\n【联系方式..."
                                                        }]
                                                    },
                                                    "publishedTimeText": {
                                                        "simpleText": "1 month ago"
                                                    },
                                                    "lengthText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "8 minutes, 4 seconds"
                                                            }
                                                        },
                                                        "simpleText": "8:04"
                                                    },
                                                    "viewCountText": {
                                                        "simpleText": "47 views"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CIoBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28WhhVQ21qZGh3TUdTdXQ4bVoxQ3FuUmpqVXeaAQYQ8jgY4AeqARpVVUxGbWpkaHdNR1N1dDhtWjFDcW5SampVdw==",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "url": "/watch?v=3NVEwECuDTM",
                                                                "webPageType": "WEB_PAGE_TYPE_WATCH",
                                                                "rootVe": 3832
                                                            }
                                                        },
                                                        "watchEndpoint": {
                                                            "videoId": "3NVEwECuDTM",
                                                            "watchEndpointSupportedOnesieConfig": {
                                                                "html5PlaybackOnesieConfig": {
                                                                    "commonConfig": {
                                                                        "url": "https://rr6---sn-cnoa-jv3s.googlevideo.com/initplayback?source=youtube\u0026oeis=1\u0026c=WEB\u0026oad=3200\u0026ovd=3200\u0026oaad=11000\u0026oavd=11000\u0026ocs=700\u0026oewis=1\u0026oputc=1\u0026ofpcc=1\u0026msp=1\u0026odepv=1\u0026id=dcd544c040ae0d33\u0026ip=117.213.17.165\u0026initcwndbps=977500\u0026mt=1679566621\u0026oweuc=\u0026pxtags=Cg4KAnR4EggyNDQyODQxNQ\u0026rxtags=Cg4KAnR4EggyNDQyODQxNA%2CCg4KAnR4EggyNDQyODQxNQ%2CCg4KAnR4EggyNDQyODQxNg%2CCg4KAnR4EggyNDQyODQxNw%2CCg4KAnR4EggyNDQyODQxOA%2CCg4KAnR4EggyNDQyODQxOQ%2CCg4KAnR4EggyNDQyODQyMA"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CIoBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28QLOauIWEmNHq3AGqARpVVUxGbWpkaHdNR1N1dDhtWjFDcW5SampVdw==",
                                                    "showActionMenu": false,
                                                    "shortViewCountText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "47 views"
                                                            }
                                                        },
                                                        "simpleText": "47 views"
                                                    },
                                                    "menu": {
                                                        "menuRenderer": {
                                                            "items": [{
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Add to queue"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "ADD_TO_QUEUE_TAIL"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CI4BEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true
                                                                            }
                                                                        },
                                                                        "signalServiceEndpoint": {
                                                                            "signal": "CLIENT_SIGNAL",
                                                                            "actions": [{
                                                                                "clickTrackingParams": "CI4BEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "addToPlaylistCommand": {
                                                                                    "openMiniplayer": true,
                                                                                    "videoId": "3NVEwECuDTM",
                                                                                    "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                                    "onCreateListCommand": {
                                                                                        "clickTrackingParams": "CI4BEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                        "commandMetadata": {
                                                                                            "webCommandMetadata": {
                                                                                                "sendPost": true,
                                                                                                "apiUrl": "/youtubei/v1/playlist/create"
                                                                                            }
                                                                                        },
                                                                                        "createPlaylistServiceEndpoint": {
                                                                                            "videoIds": ["3NVEwECuDTM"],
                                                                                            "params": "CAQ%3D"
                                                                                        }
                                                                                    },
                                                                                    "videoIds": ["3NVEwECuDTM"]
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CI4BEP6YBBgGIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                                }
                                                            }, {
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Share"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "SHARE"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CIoBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true,
                                                                                "apiUrl": "/youtubei/v1/share/get_share_panel"
                                                                            }
                                                                        },
                                                                        "shareEntityServiceEndpoint": {
                                                                            "serializedShareEntity": "CgszTlZFd0VDdURUTQ%3D%3D",
                                                                            "commands": [{
                                                                                "clickTrackingParams": "CIoBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "openPopupAction": {
                                                                                    "popup": {
                                                                                        "unifiedSharePanelRenderer": {
                                                                                            "trackingParams": "CI0BEI5iIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                            "showLoadingSpinner": true
                                                                                        }
                                                                                    },
                                                                                    "popupType": "DIALOG",
                                                                                    "beReused": true
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CIoBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                                }
                                                            }],
                                                            "trackingParams": "CIoBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                            "accessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Action menu"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "thumbnailOverlays": [{
                                                        "thumbnailOverlayTimeStatusRenderer": {
                                                            "text": {
                                                                "accessibility": {
                                                                    "accessibilityData": {
                                                                        "label": "8 minutes, 4 seconds"
                                                                    }
                                                                },
                                                                "simpleText": "8:04"
                                                            },
                                                            "style": "DEFAULT"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "isToggled": false,
                                                            "untoggledIcon": {
                                                                "iconType": "WATCH_LATER"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "CHECK"
                                                            },
                                                            "untoggledTooltip": "Watch later",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CIwBEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "addedVideoId": "3NVEwECuDTM",
                                                                        "action": "ACTION_ADD_VIDEO"
                                                                    }]
                                                                }
                                                            },
                                                            "toggledServiceEndpoint": {
                                                                "clickTrackingParams": "CIwBEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "action": "ACTION_REMOVE_VIDEO_BY_VIDEO_ID",
                                                                        "removedVideoId": "3NVEwECuDTM"
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Watch later"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CIwBEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "untoggledIcon": {
                                                                "iconType": "ADD_TO_QUEUE_TAIL"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "PLAYLIST_ADD_CHECK"
                                                            },
                                                            "untoggledTooltip": "Add to queue",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CIsBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true
                                                                    }
                                                                },
                                                                "signalServiceEndpoint": {
                                                                    "signal": "CLIENT_SIGNAL",
                                                                    "actions": [{
                                                                        "clickTrackingParams": "CIsBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "addToPlaylistCommand": {
                                                                            "openMiniplayer": true,
                                                                            "videoId": "3NVEwECuDTM",
                                                                            "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                            "onCreateListCommand": {
                                                                                "clickTrackingParams": "CIsBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "commandMetadata": {
                                                                                    "webCommandMetadata": {
                                                                                        "sendPost": true,
                                                                                        "apiUrl": "/youtubei/v1/playlist/create"
                                                                                    }
                                                                                },
                                                                                "createPlaylistServiceEndpoint": {
                                                                                    "videoIds": ["3NVEwECuDTM"],
                                                                                    "params": "CAQ%3D"
                                                                                }
                                                                            },
                                                                            "videoIds": ["3NVEwECuDTM"]
                                                                        }
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Add to queue"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CIsBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayNowPlayingRenderer": {
                                                            "text": {
                                                                "runs": [{
                                                                    "text": "Now playing"
                                                                }]
                                                            }
                                                        }
                                                    }],
                                                    "richThumbnail": {
                                                        "movingThumbnailRenderer": {
                                                            "movingThumbnailDetails": {
                                                                "thumbnails": [{
                                                                    "url": "https://i.ytimg.com/an_webp/3NVEwECuDTM/mqdefault_6s.webp?du=3000\u0026sqp=CKi38KAG\u0026rs=AOn4CLA__XTY33va822S4b4eYE2Tru5UXQ",
                                                                    "width": 320,
                                                                    "height": 180
                                                                }],
                                                                "logAsMovingThumbnail": true
                                                            },
                                                            "enableHoveredLogging": true,
                                                            "enableOverlay": true
                                                        }
                                                    }
                                                }
                                            },
                                            "trackingParams": "CIkBEJmNBRgLIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                        }
                                    }, {
                                        "richItemRenderer": {
                                            "content": {
                                                "videoRenderer": {
                                                    "videoId": "7bBrNyBddRk",
                                                    "thumbnail": {
                                                        "thumbnails": [{
                                                            "url": "https://i.ytimg.com/vi/7bBrNyBddRk/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLDLBbe8P6kQsf67n1UbM8yG2bDyOw",
                                                            "width": 168,
                                                            "height": 94
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/7bBrNyBddRk/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLAJZmgok8-3BUIhcV6bLEKLvWOrnA",
                                                            "width": 196,
                                                            "height": 110
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/7bBrNyBddRk/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLC9S1JHQeb3C5XWN5Pb8oQSUsgJOw",
                                                            "width": 246,
                                                            "height": 138
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/7bBrNyBddRk/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLA3sSg-ZOF0goOrjj8eIbTLFn_Q5A",
                                                            "width": 336,
                                                            "height": 188
                                                        }]
                                                    },
                                                    "title": {
                                                        "runs": [{
                                                            "text": "3.一起创建我们第一个Github Actions"
                                                        }],
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "3.一起创建我们第一个Github Actions by 麦兜搞IT 2 months ago 12 minutes, 40 seconds 126 views"
                                                            }
                                                        }
                                                    },
                                                    "descriptionSnippet": {
                                                        "runs": [{
                                                            "text": "===============================\n【我的一些在线课程】\n\nUdemy：https://github.com/udemy-course\nHiskio：https://hiskio.com/search?word=peng%20xiao\u0026sort=attendance\u0026page=1\n\n【联系方式..."
                                                        }]
                                                    },
                                                    "publishedTimeText": {
                                                        "simpleText": "2 months ago"
                                                    },
                                                    "lengthText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "12 minutes, 40 seconds"
                                                            }
                                                        },
                                                        "simpleText": "12:40"
                                                    },
                                                    "viewCountText": {
                                                        "simpleText": "126 views"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CIQBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28WhhVQ21qZGh3TUdTdXQ4bVoxQ3FuUmpqVXeaAQYQ8jgY4AeqARpVVUxGbWpkaHdNR1N1dDhtWjFDcW5SampVdw==",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "url": "/watch?v=7bBrNyBddRk",
                                                                "webPageType": "WEB_PAGE_TYPE_WATCH",
                                                                "rootVe": 3832
                                                            }
                                                        },
                                                        "watchEndpoint": {
                                                            "videoId": "7bBrNyBddRk",
                                                            "watchEndpointSupportedOnesieConfig": {
                                                                "html5PlaybackOnesieConfig": {
                                                                    "commonConfig": {
                                                                        "url": "https://rr3---sn-cnoa-jv3l.googlevideo.com/initplayback?source=youtube\u0026oeis=1\u0026c=WEB\u0026oad=3200\u0026ovd=3200\u0026oaad=11000\u0026oavd=11000\u0026ocs=700\u0026oewis=1\u0026oputc=1\u0026ofpcc=1\u0026msp=1\u0026odepv=1\u0026id=edb06b37205d7519\u0026ip=117.213.17.165\u0026initcwndbps=1068750\u0026mt=1679566621\u0026oweuc=\u0026pxtags=Cg4KAnR4EggyNDQyODQxNQ\u0026rxtags=Cg4KAnR4EggyNDQyODQxNA%2CCg4KAnR4EggyNDQyODQxNQ%2CCg4KAnR4EggyNDQyODQxNg%2CCg4KAnR4EggyNDQyODQxNw%2CCg4KAnR4EggyNDQyODQxOA%2CCg4KAnR4EggyNDQyODQxOQ%2CCg4KAnR4EggyNDQyODQyMA"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CIQBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28QJnq9YLy5prY7QGqARpVVUxGbWpkaHdNR1N1dDhtWjFDcW5SampVdw==",
                                                    "showActionMenu": false,
                                                    "shortViewCountText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "126 views"
                                                            }
                                                        },
                                                        "simpleText": "126 views"
                                                    },
                                                    "menu": {
                                                        "menuRenderer": {
                                                            "items": [{
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Add to queue"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "ADD_TO_QUEUE_TAIL"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CIgBEP6YBBgFIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true
                                                                            }
                                                                        },
                                                                        "signalServiceEndpoint": {
                                                                            "signal": "CLIENT_SIGNAL",
                                                                            "actions": [{
                                                                                "clickTrackingParams": "CIgBEP6YBBgFIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "addToPlaylistCommand": {
                                                                                    "openMiniplayer": true,
                                                                                    "videoId": "7bBrNyBddRk",
                                                                                    "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                                    "onCreateListCommand": {
                                                                                        "clickTrackingParams": "CIgBEP6YBBgFIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                        "commandMetadata": {
                                                                                            "webCommandMetadata": {
                                                                                                "sendPost": true,
                                                                                                "apiUrl": "/youtubei/v1/playlist/create"
                                                                                            }
                                                                                        },
                                                                                        "createPlaylistServiceEndpoint": {
                                                                                            "videoIds": ["7bBrNyBddRk"],
                                                                                            "params": "CAQ%3D"
                                                                                        }
                                                                                    },
                                                                                    "videoIds": ["7bBrNyBddRk"]
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CIgBEP6YBBgFIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                                }
                                                            }, {
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Share"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "SHARE"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CIQBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true,
                                                                                "apiUrl": "/youtubei/v1/share/get_share_panel"
                                                                            }
                                                                        },
                                                                        "shareEntityServiceEndpoint": {
                                                                            "serializedShareEntity": "Cgs3YkJyTnlCZGRSaw%3D%3D",
                                                                            "commands": [{
                                                                                "clickTrackingParams": "CIQBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "openPopupAction": {
                                                                                    "popup": {
                                                                                        "unifiedSharePanelRenderer": {
                                                                                            "trackingParams": "CIcBEI5iIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                            "showLoadingSpinner": true
                                                                                        }
                                                                                    },
                                                                                    "popupType": "DIALOG",
                                                                                    "beReused": true
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CIQBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                                }
                                                            }],
                                                            "trackingParams": "CIQBENwwIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                            "accessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Action menu"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "thumbnailOverlays": [{
                                                        "thumbnailOverlayTimeStatusRenderer": {
                                                            "text": {
                                                                "accessibility": {
                                                                    "accessibilityData": {
                                                                        "label": "12 minutes, 40 seconds"
                                                                    }
                                                                },
                                                                "simpleText": "12:40"
                                                            },
                                                            "style": "DEFAULT"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "isToggled": false,
                                                            "untoggledIcon": {
                                                                "iconType": "WATCH_LATER"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "CHECK"
                                                            },
                                                            "untoggledTooltip": "Watch later",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CIYBEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "addedVideoId": "7bBrNyBddRk",
                                                                        "action": "ACTION_ADD_VIDEO"
                                                                    }]
                                                                }
                                                            },
                                                            "toggledServiceEndpoint": {
                                                                "clickTrackingParams": "CIYBEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "action": "ACTION_REMOVE_VIDEO_BY_VIDEO_ID",
                                                                        "removedVideoId": "7bBrNyBddRk"
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Watch later"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CIYBEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "untoggledIcon": {
                                                                "iconType": "ADD_TO_QUEUE_TAIL"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "PLAYLIST_ADD_CHECK"
                                                            },
                                                            "untoggledTooltip": "Add to queue",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CIUBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true
                                                                    }
                                                                },
                                                                "signalServiceEndpoint": {
                                                                    "signal": "CLIENT_SIGNAL",
                                                                    "actions": [{
                                                                        "clickTrackingParams": "CIUBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "addToPlaylistCommand": {
                                                                            "openMiniplayer": true,
                                                                            "videoId": "7bBrNyBddRk",
                                                                            "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                            "onCreateListCommand": {
                                                                                "clickTrackingParams": "CIUBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "commandMetadata": {
                                                                                    "webCommandMetadata": {
                                                                                        "sendPost": true,
                                                                                        "apiUrl": "/youtubei/v1/playlist/create"
                                                                                    }
                                                                                },
                                                                                "createPlaylistServiceEndpoint": {
                                                                                    "videoIds": ["7bBrNyBddRk"],
                                                                                    "params": "CAQ%3D"
                                                                                }
                                                                            },
                                                                            "videoIds": ["7bBrNyBddRk"]
                                                                        }
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Add to queue"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CIUBEMfsBBgCIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayNowPlayingRenderer": {
                                                            "text": {
                                                                "runs": [{
                                                                    "text": "Now playing"
                                                                }]
                                                            }
                                                        }
                                                    }]
                                                }
                                            },
                                            "trackingParams": "CIMBEJmNBRgMIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                        }
                                    }, {
                                        "richItemRenderer": {
                                            "content": {
                                                "videoRenderer": {
                                                    "videoId": "w-Rz_U2KToU",
                                                    "thumbnail": {
                                                        "thumbnails": [{
                                                            "url": "https://i.ytimg.com/vi/w-Rz_U2KToU/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLC8jOuCCd80xTC3ZhcWmcLh_SMi0Q",
                                                            "width": 168,
                                                            "height": 94
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/w-Rz_U2KToU/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLBG-vXj7UheH-MOjos8XoDgFJsGbQ",
                                                            "width": 196,
                                                            "height": 110
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/w-Rz_U2KToU/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLBcoVnZJDdWr-eO1uF1nTrITZ7LhA",
                                                            "width": 246,
                                                            "height": 138
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/w-Rz_U2KToU/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLB0e-iK7j55DDlqqnreApqQIshZPA",
                                                            "width": 336,
                                                            "height": 188
                                                        }]
                                                    },
                                                    "title": {
                                                        "runs": [{
                                                            "text": "2. GitHub Action Components | GitHub Action里的核心概念"
                                                        }],
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "2. GitHub Action Components | GitHub Action里的核心概念 by 麦兜搞IT 2 months ago 8 minutes, 4 seconds 100 views"
                                                            }
                                                        }
                                                    },
                                                    "descriptionSnippet": {
                                                        "runs": [{
                                                            "text": "===============================\n【我的一些在线课程】\n\nUdemy：https://github.com/udemy-course\nHiskio：https://hiskio.com/search?word=peng%20xiao\u0026sort=attendance\u0026page=1\n\n【联系方式..."
                                                        }]
                                                    },
                                                    "publishedTimeText": {
                                                        "simpleText": "2 months ago"
                                                    },
                                                    "lengthText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "8 minutes, 4 seconds"
                                                            }
                                                        },
                                                        "simpleText": "8:04"
                                                    },
                                                    "viewCountText": {
                                                        "simpleText": "100 views"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CH4Q3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxaGFVDbWpkaHdNR1N1dDhtWjFDcW5SampVd5oBBhDyOBjgB6oBGlVVTEZtamRod01HU3V0OG1aMUNxblJqalV3",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "url": "/watch?v=w-Rz_U2KToU",
                                                                "webPageType": "WEB_PAGE_TYPE_WATCH",
                                                                "rootVe": 3832
                                                            }
                                                        },
                                                        "watchEndpoint": {
                                                            "videoId": "w-Rz_U2KToU",
                                                            "watchEndpointSupportedOnesieConfig": {
                                                                "html5PlaybackOnesieConfig": {
                                                                    "commonConfig": {
                                                                        "url": "https://rr1---sn-cnoa-jv3s.googlevideo.com/initplayback?source=youtube\u0026oeis=1\u0026c=WEB\u0026oad=3200\u0026ovd=3200\u0026oaad=11000\u0026oavd=11000\u0026ocs=700\u0026oewis=1\u0026oputc=1\u0026ofpcc=1\u0026msp=1\u0026odepv=1\u0026id=c3e473fd4d8a4e85\u0026ip=117.213.17.165\u0026initcwndbps=977500\u0026mt=1679566621\u0026oweuc=\u0026pxtags=Cg4KAnR4EggyNDQyODQxNQ\u0026rxtags=Cg4KAnR4EggyNDQyODQxNA%2CCg4KAnR4EggyNDQyODQxNQ%2CCg4KAnR4EggyNDQyODQxNg%2CCg4KAnR4EggyNDQyODQxNw%2CCg4KAnR4EggyNDQyODQxOA%2CCg4KAnR4EggyNDQyODQxOQ%2CCg4KAnR4EggyNDQyODQyMA"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CH4Q3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxAhZ2p7NT_nPLDAaoBGlVVTEZtamRod01HU3V0OG1aMUNxblJqalV3",
                                                    "showActionMenu": false,
                                                    "shortViewCountText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "100 views"
                                                            }
                                                        },
                                                        "simpleText": "100 views"
                                                    },
                                                    "menu": {
                                                        "menuRenderer": {
                                                            "items": [{
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Add to queue"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "ADD_TO_QUEUE_TAIL"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CIIBEP6YBBgFIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true
                                                                            }
                                                                        },
                                                                        "signalServiceEndpoint": {
                                                                            "signal": "CLIENT_SIGNAL",
                                                                            "actions": [{
                                                                                "clickTrackingParams": "CIIBEP6YBBgFIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                "addToPlaylistCommand": {
                                                                                    "openMiniplayer": true,
                                                                                    "videoId": "w-Rz_U2KToU",
                                                                                    "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                                    "onCreateListCommand": {
                                                                                        "clickTrackingParams": "CIIBEP6YBBgFIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                        "commandMetadata": {
                                                                                            "webCommandMetadata": {
                                                                                                "sendPost": true,
                                                                                                "apiUrl": "/youtubei/v1/playlist/create"
                                                                                            }
                                                                                        },
                                                                                        "createPlaylistServiceEndpoint": {
                                                                                            "videoIds": ["w-Rz_U2KToU"],
                                                                                            "params": "CAQ%3D"
                                                                                        }
                                                                                    },
                                                                                    "videoIds": ["w-Rz_U2KToU"]
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CIIBEP6YBBgFIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                                }
                                                            }, {
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Share"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "SHARE"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CH4Q3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true,
                                                                                "apiUrl": "/youtubei/v1/share/get_share_panel"
                                                                            }
                                                                        },
                                                                        "shareEntityServiceEndpoint": {
                                                                            "serializedShareEntity": "Cgt3LVJ6X1UyS1RvVQ%3D%3D",
                                                                            "commands": [{
                                                                                "clickTrackingParams": "CH4Q3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "openPopupAction": {
                                                                                    "popup": {
                                                                                        "unifiedSharePanelRenderer": {
                                                                                            "trackingParams": "CIEBEI5iIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                                            "showLoadingSpinner": true
                                                                                        }
                                                                                    },
                                                                                    "popupType": "DIALOG",
                                                                                    "beReused": true
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CH4Q3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }],
                                                            "trackingParams": "CH4Q3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                            "accessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Action menu"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "thumbnailOverlays": [{
                                                        "thumbnailOverlayTimeStatusRenderer": {
                                                            "text": {
                                                                "accessibility": {
                                                                    "accessibilityData": {
                                                                        "label": "8 minutes, 4 seconds"
                                                                    }
                                                                },
                                                                "simpleText": "8:04"
                                                            },
                                                            "style": "DEFAULT"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "isToggled": false,
                                                            "untoggledIcon": {
                                                                "iconType": "WATCH_LATER"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "CHECK"
                                                            },
                                                            "untoggledTooltip": "Watch later",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CIABEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "addedVideoId": "w-Rz_U2KToU",
                                                                        "action": "ACTION_ADD_VIDEO"
                                                                    }]
                                                                }
                                                            },
                                                            "toggledServiceEndpoint": {
                                                                "clickTrackingParams": "CIABEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "action": "ACTION_REMOVE_VIDEO_BY_VIDEO_ID",
                                                                        "removedVideoId": "w-Rz_U2KToU"
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Watch later"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CIABEPnnAxgBIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "untoggledIcon": {
                                                                "iconType": "ADD_TO_QUEUE_TAIL"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "PLAYLIST_ADD_CHECK"
                                                            },
                                                            "untoggledTooltip": "Add to queue",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CH8Qx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true
                                                                    }
                                                                },
                                                                "signalServiceEndpoint": {
                                                                    "signal": "CLIENT_SIGNAL",
                                                                    "actions": [{
                                                                        "clickTrackingParams": "CH8Qx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "addToPlaylistCommand": {
                                                                            "openMiniplayer": true,
                                                                            "videoId": "w-Rz_U2KToU",
                                                                            "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                            "onCreateListCommand": {
                                                                                "clickTrackingParams": "CH8Qx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "commandMetadata": {
                                                                                    "webCommandMetadata": {
                                                                                        "sendPost": true,
                                                                                        "apiUrl": "/youtubei/v1/playlist/create"
                                                                                    }
                                                                                },
                                                                                "createPlaylistServiceEndpoint": {
                                                                                    "videoIds": ["w-Rz_U2KToU"],
                                                                                    "params": "CAQ%3D"
                                                                                }
                                                                            },
                                                                            "videoIds": ["w-Rz_U2KToU"]
                                                                        }
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Add to queue"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CH8Qx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayNowPlayingRenderer": {
                                                            "text": {
                                                                "runs": [{
                                                                    "text": "Now playing"
                                                                }]
                                                            }
                                                        }
                                                    }]
                                                }
                                            },
                                            "trackingParams": "CH0QmY0FGA0iEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                        }
                                    }, {
                                        "richItemRenderer": {
                                            "content": {
                                                "videoRenderer": {
                                                    "videoId": "NkzqrPu0xRo",
                                                    "thumbnail": {
                                                        "thumbnails": [{
                                                            "url": "https://i.ytimg.com/vi/NkzqrPu0xRo/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLBTeAuemWAshg5K9fQeEJMvwysFEA",
                                                            "width": 168,
                                                            "height": 94
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/NkzqrPu0xRo/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLDKRwmEM0Yq0f8N3532MQtw8ua4zg",
                                                            "width": 196,
                                                            "height": 110
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/NkzqrPu0xRo/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLDn31oENGGepGQJgmcAzcZ2-aZU_g",
                                                            "width": 246,
                                                            "height": 138
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/NkzqrPu0xRo/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLB7ryfVAnuTu0Cw1-rbKuz5T7DKNw",
                                                            "width": 336,
                                                            "height": 188
                                                        }]
                                                    },
                                                    "title": {
                                                        "runs": [{
                                                            "text": "1. 什么是GitHub Action？免费么？可以用来做什么？"
                                                        }],
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "1. 什么是GitHub Action？免费么？可以用来做什么？ by 麦兜搞IT 2 months ago 7 minutes, 54 seconds 350 views"
                                                            }
                                                        }
                                                    },
                                                    "descriptionSnippet": {
                                                        "runs": [{
                                                            "text": "===============================\n【我的一些在线课程】\n\nUdemy：https://github.com/udemy-course\nHiskio：https://hiskio.com/search?word=peng%20xiao\u0026sort=attendance\u0026page=1\n\n【联系方式..."
                                                        }]
                                                    },
                                                    "publishedTimeText": {
                                                        "simpleText": "2 months ago"
                                                    },
                                                    "lengthText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "7 minutes, 54 seconds"
                                                            }
                                                        },
                                                        "simpleText": "7:54"
                                                    },
                                                    "viewCountText": {
                                                        "simpleText": "350 views"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CHgQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxaGFVDbWpkaHdNR1N1dDhtWjFDcW5SampVd5oBBhDyOBjgB6oBGlVVTEZtamRod01HU3V0OG1aMUNxblJqalV3",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "url": "/watch?v=NkzqrPu0xRo",
                                                                "webPageType": "WEB_PAGE_TYPE_WATCH",
                                                                "rootVe": 3832
                                                            }
                                                        },
                                                        "watchEndpoint": {
                                                            "videoId": "NkzqrPu0xRo",
                                                            "watchEndpointSupportedOnesieConfig": {
                                                                "html5PlaybackOnesieConfig": {
                                                                    "commonConfig": {
                                                                        "url": "https://rr3---sn-cnoa-jv3s.googlevideo.com/initplayback?source=youtube\u0026oeis=1\u0026c=WEB\u0026oad=3200\u0026ovd=3200\u0026oaad=11000\u0026oavd=11000\u0026ocs=700\u0026oewis=1\u0026oputc=1\u0026ofpcc=1\u0026msp=1\u0026odepv=1\u0026id=364ceaacfbb4c51a\u0026ip=117.213.17.165\u0026initcwndbps=977500\u0026mt=1679566621\u0026oweuc=\u0026pxtags=Cg4KAnR4EggyNDQyODQxNQ\u0026rxtags=Cg4KAnR4EggyNDQyODQxNA%2CCg4KAnR4EggyNDQyODQxNQ%2CCg4KAnR4EggyNDQyODQxNg%2CCg4KAnR4EggyNDQyODQxNw%2CCg4KAnR4EggyNDQyODQxOA%2CCg4KAnR4EggyNDQyODQxOQ%2CCg4KAnR4EggyNDQyODQyMA"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CHgQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxAmorT3c_VuqY2qgEaVVVMRm1qZGh3TUdTdXQ4bVoxQ3FuUmpqVXc=",
                                                    "showActionMenu": false,
                                                    "shortViewCountText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "350 views"
                                                            }
                                                        },
                                                        "simpleText": "350 views"
                                                    },
                                                    "menu": {
                                                        "menuRenderer": {
                                                            "items": [{
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Add to queue"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "ADD_TO_QUEUE_TAIL"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CHwQ_pgEGAUiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true
                                                                            }
                                                                        },
                                                                        "signalServiceEndpoint": {
                                                                            "signal": "CLIENT_SIGNAL",
                                                                            "actions": [{
                                                                                "clickTrackingParams": "CHwQ_pgEGAUiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "addToPlaylistCommand": {
                                                                                    "openMiniplayer": true,
                                                                                    "videoId": "NkzqrPu0xRo",
                                                                                    "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                                    "onCreateListCommand": {
                                                                                        "clickTrackingParams": "CHwQ_pgEGAUiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                        "commandMetadata": {
                                                                                            "webCommandMetadata": {
                                                                                                "sendPost": true,
                                                                                                "apiUrl": "/youtubei/v1/playlist/create"
                                                                                            }
                                                                                        },
                                                                                        "createPlaylistServiceEndpoint": {
                                                                                            "videoIds": ["NkzqrPu0xRo"],
                                                                                            "params": "CAQ%3D"
                                                                                        }
                                                                                    },
                                                                                    "videoIds": ["NkzqrPu0xRo"]
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CHwQ_pgEGAUiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }, {
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Share"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "SHARE"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CHgQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true,
                                                                                "apiUrl": "/youtubei/v1/share/get_share_panel"
                                                                            }
                                                                        },
                                                                        "shareEntityServiceEndpoint": {
                                                                            "serializedShareEntity": "CgtOa3pxclB1MHhSbw%3D%3D",
                                                                            "commands": [{
                                                                                "clickTrackingParams": "CHgQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "openPopupAction": {
                                                                                    "popup": {
                                                                                        "unifiedSharePanelRenderer": {
                                                                                            "trackingParams": "CHsQjmIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                            "showLoadingSpinner": true
                                                                                        }
                                                                                    },
                                                                                    "popupType": "DIALOG",
                                                                                    "beReused": true
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CHgQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }],
                                                            "trackingParams": "CHgQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                            "accessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Action menu"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "thumbnailOverlays": [{
                                                        "thumbnailOverlayTimeStatusRenderer": {
                                                            "text": {
                                                                "accessibility": {
                                                                    "accessibilityData": {
                                                                        "label": "7 minutes, 54 seconds"
                                                                    }
                                                                },
                                                                "simpleText": "7:54"
                                                            },
                                                            "style": "DEFAULT"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "isToggled": false,
                                                            "untoggledIcon": {
                                                                "iconType": "WATCH_LATER"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "CHECK"
                                                            },
                                                            "untoggledTooltip": "Watch later",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CHoQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "addedVideoId": "NkzqrPu0xRo",
                                                                        "action": "ACTION_ADD_VIDEO"
                                                                    }]
                                                                }
                                                            },
                                                            "toggledServiceEndpoint": {
                                                                "clickTrackingParams": "CHoQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "action": "ACTION_REMOVE_VIDEO_BY_VIDEO_ID",
                                                                        "removedVideoId": "NkzqrPu0xRo"
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Watch later"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CHoQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "untoggledIcon": {
                                                                "iconType": "ADD_TO_QUEUE_TAIL"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "PLAYLIST_ADD_CHECK"
                                                            },
                                                            "untoggledTooltip": "Add to queue",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CHkQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true
                                                                    }
                                                                },
                                                                "signalServiceEndpoint": {
                                                                    "signal": "CLIENT_SIGNAL",
                                                                    "actions": [{
                                                                        "clickTrackingParams": "CHkQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "addToPlaylistCommand": {
                                                                            "openMiniplayer": true,
                                                                            "videoId": "NkzqrPu0xRo",
                                                                            "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                            "onCreateListCommand": {
                                                                                "clickTrackingParams": "CHkQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "commandMetadata": {
                                                                                    "webCommandMetadata": {
                                                                                        "sendPost": true,
                                                                                        "apiUrl": "/youtubei/v1/playlist/create"
                                                                                    }
                                                                                },
                                                                                "createPlaylistServiceEndpoint": {
                                                                                    "videoIds": ["NkzqrPu0xRo"],
                                                                                    "params": "CAQ%3D"
                                                                                }
                                                                            },
                                                                            "videoIds": ["NkzqrPu0xRo"]
                                                                        }
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Add to queue"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CHkQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayNowPlayingRenderer": {
                                                            "text": {
                                                                "runs": [{
                                                                    "text": "Now playing"
                                                                }]
                                                            }
                                                        }
                                                    }]
                                                }
                                            },
                                            "trackingParams": "CHcQmY0FGA4iEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                        }
                                    }, {
                                        "richItemRenderer": {
                                            "content": {
                                                "videoRenderer": {
                                                    "videoId": "xZGvrpHof1Y",
                                                    "thumbnail": {
                                                        "thumbnails": [{
                                                            "url": "https://i.ytimg.com/vi/xZGvrpHof1Y/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLCwp358_Me58c24XGt_L4dqL8IOrw",
                                                            "width": 168,
                                                            "height": 94
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/xZGvrpHof1Y/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLD1SI-xiaHLT-kXex1vwS6sSBKxKg",
                                                            "width": 196,
                                                            "height": 110
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/xZGvrpHof1Y/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLCnF9Mgc43ZX-teILKHSlRAJO1Tsg",
                                                            "width": 246,
                                                            "height": 138
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/xZGvrpHof1Y/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLCg7APVRKB7oiPgiJSRkCOCZaGMnQ",
                                                            "width": 336,
                                                            "height": 188
                                                        }]
                                                    },
                                                    "title": {
                                                        "runs": [{
                                                            "text": "GitHub CodeSpaces和Dev Container系列——Docker in Docker"
                                                        }],
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "GitHub CodeSpaces和Dev Container系列——Docker in Docker by 麦兜搞IT 2 months ago 11 minutes, 35 seconds 110 views"
                                                            }
                                                        }
                                                    },
                                                    "descriptionSnippet": {
                                                        "runs": [{
                                                            "text": "https://github.com/xiaopeng163/nginx-flask-mysql\n\n===============================\n【我的一些在线课程】\n\nUdemy：https://github.com/udemy-course\nHiskio：https://hiskio.com/search?word=pen..."
                                                        }]
                                                    },
                                                    "publishedTimeText": {
                                                        "simpleText": "2 months ago"
                                                    },
                                                    "lengthText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "11 minutes, 35 seconds"
                                                            }
                                                        },
                                                        "simpleText": "11:35"
                                                    },
                                                    "viewCountText": {
                                                        "simpleText": "110 views"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CHIQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxaGFVDbWpkaHdNR1N1dDhtWjFDcW5SampVd5oBBhDyOBjgB6oBGlVVTEZtamRod01HU3V0OG1aMUNxblJqalV3",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "url": "/watch?v=xZGvrpHof1Y",
                                                                "webPageType": "WEB_PAGE_TYPE_WATCH",
                                                                "rootVe": 3832
                                                            }
                                                        },
                                                        "watchEndpoint": {
                                                            "videoId": "xZGvrpHof1Y",
                                                            "watchEndpointSupportedOnesieConfig": {
                                                                "html5PlaybackOnesieConfig": {
                                                                    "commonConfig": {
                                                                        "url": "https://rr3---sn-cnoa-jv3s.googlevideo.com/initplayback?source=youtube\u0026oeis=1\u0026c=WEB\u0026oad=3200\u0026ovd=3200\u0026oaad=11000\u0026oavd=11000\u0026ocs=700\u0026oewis=1\u0026oputc=1\u0026ofpcc=1\u0026msp=1\u0026odepv=1\u0026id=c591afae91e87f56\u0026ip=117.213.17.165\u0026initcwndbps=977500\u0026mt=1679566621\u0026oweuc=\u0026pxtags=Cg4KAnR4EggyNDQyODQxNQ\u0026rxtags=Cg4KAnR4EggyNDQyODQxNA%2CCg4KAnR4EggyNDQyODQxNQ%2CCg4KAnR4EggyNDQyODQxNg%2CCg4KAnR4EggyNDQyODQxNw%2CCg4KAnR4EggyNDQyODQxOA%2CCg4KAnR4EggyNDQyODQxOQ%2CCg4KAnR4EggyNDQyODQyMA"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CHIQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxA1v6hj-n168jFAaoBGlVVTEZtamRod01HU3V0OG1aMUNxblJqalV3",
                                                    "showActionMenu": false,
                                                    "shortViewCountText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "110 views"
                                                            }
                                                        },
                                                        "simpleText": "110 views"
                                                    },
                                                    "menu": {
                                                        "menuRenderer": {
                                                            "items": [{
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Add to queue"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "ADD_TO_QUEUE_TAIL"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CHYQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true
                                                                            }
                                                                        },
                                                                        "signalServiceEndpoint": {
                                                                            "signal": "CLIENT_SIGNAL",
                                                                            "actions": [{
                                                                                "clickTrackingParams": "CHYQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "addToPlaylistCommand": {
                                                                                    "openMiniplayer": true,
                                                                                    "videoId": "xZGvrpHof1Y",
                                                                                    "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                                    "onCreateListCommand": {
                                                                                        "clickTrackingParams": "CHYQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                        "commandMetadata": {
                                                                                            "webCommandMetadata": {
                                                                                                "sendPost": true,
                                                                                                "apiUrl": "/youtubei/v1/playlist/create"
                                                                                            }
                                                                                        },
                                                                                        "createPlaylistServiceEndpoint": {
                                                                                            "videoIds": ["xZGvrpHof1Y"],
                                                                                            "params": "CAQ%3D"
                                                                                        }
                                                                                    },
                                                                                    "videoIds": ["xZGvrpHof1Y"]
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CHYQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }, {
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Share"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "SHARE"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CHIQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true,
                                                                                "apiUrl": "/youtubei/v1/share/get_share_panel"
                                                                            }
                                                                        },
                                                                        "shareEntityServiceEndpoint": {
                                                                            "serializedShareEntity": "Cgt4Wkd2cnBIb2YxWQ%3D%3D",
                                                                            "commands": [{
                                                                                "clickTrackingParams": "CHIQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "openPopupAction": {
                                                                                    "popup": {
                                                                                        "unifiedSharePanelRenderer": {
                                                                                            "trackingParams": "CHUQjmIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                            "showLoadingSpinner": true
                                                                                        }
                                                                                    },
                                                                                    "popupType": "DIALOG",
                                                                                    "beReused": true
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CHIQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }],
                                                            "trackingParams": "CHIQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                            "accessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Action menu"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "thumbnailOverlays": [{
                                                        "thumbnailOverlayTimeStatusRenderer": {
                                                            "text": {
                                                                "accessibility": {
                                                                    "accessibilityData": {
                                                                        "label": "11 minutes, 35 seconds"
                                                                    }
                                                                },
                                                                "simpleText": "11:35"
                                                            },
                                                            "style": "DEFAULT"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "isToggled": false,
                                                            "untoggledIcon": {
                                                                "iconType": "WATCH_LATER"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "CHECK"
                                                            },
                                                            "untoggledTooltip": "Watch later",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CHQQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "addedVideoId": "xZGvrpHof1Y",
                                                                        "action": "ACTION_ADD_VIDEO"
                                                                    }]
                                                                }
                                                            },
                                                            "toggledServiceEndpoint": {
                                                                "clickTrackingParams": "CHQQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "action": "ACTION_REMOVE_VIDEO_BY_VIDEO_ID",
                                                                        "removedVideoId": "xZGvrpHof1Y"
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Watch later"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CHQQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "untoggledIcon": {
                                                                "iconType": "ADD_TO_QUEUE_TAIL"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "PLAYLIST_ADD_CHECK"
                                                            },
                                                            "untoggledTooltip": "Add to queue",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CHMQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true
                                                                    }
                                                                },
                                                                "signalServiceEndpoint": {
                                                                    "signal": "CLIENT_SIGNAL",
                                                                    "actions": [{
                                                                        "clickTrackingParams": "CHMQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "addToPlaylistCommand": {
                                                                            "openMiniplayer": true,
                                                                            "videoId": "xZGvrpHof1Y",
                                                                            "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                            "onCreateListCommand": {
                                                                                "clickTrackingParams": "CHMQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "commandMetadata": {
                                                                                    "webCommandMetadata": {
                                                                                        "sendPost": true,
                                                                                        "apiUrl": "/youtubei/v1/playlist/create"
                                                                                    }
                                                                                },
                                                                                "createPlaylistServiceEndpoint": {
                                                                                    "videoIds": ["xZGvrpHof1Y"],
                                                                                    "params": "CAQ%3D"
                                                                                }
                                                                            },
                                                                            "videoIds": ["xZGvrpHof1Y"]
                                                                        }
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Add to queue"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CHMQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayNowPlayingRenderer": {
                                                            "text": {
                                                                "runs": [{
                                                                    "text": "Now playing"
                                                                }]
                                                            }
                                                        }
                                                    }],
                                                    "richThumbnail": {
                                                        "movingThumbnailRenderer": {
                                                            "movingThumbnailDetails": {
                                                                "thumbnails": [{
                                                                    "url": "https://i.ytimg.com/an_webp/xZGvrpHof1Y/mqdefault_6s.webp?du=3000\u0026sqp=CKSu8KAG\u0026rs=AOn4CLDyuYzVAYvXB7HXUmNjlc_0zK991Q",
                                                                    "width": 320,
                                                                    "height": 180
                                                                }],
                                                                "logAsMovingThumbnail": true
                                                            },
                                                            "enableHoveredLogging": true,
                                                            "enableOverlay": true
                                                        }
                                                    }
                                                }
                                            },
                                            "trackingParams": "CHEQmY0FGA8iEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                        }
                                    }, {
                                        "richItemRenderer": {
                                            "content": {
                                                "videoRenderer": {
                                                    "videoId": "26MinSlgHoE",
                                                    "thumbnail": {
                                                        "thumbnails": [{
                                                            "url": "https://i.ytimg.com/vi/26MinSlgHoE/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLCLxv8pBXaZy-uQYhn4zAh8xYbi9g",
                                                            "width": 168,
                                                            "height": 94
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/26MinSlgHoE/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLAZL4LjS5Xb2IbUx-BHX2jWvn59kg",
                                                            "width": 196,
                                                            "height": 110
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/26MinSlgHoE/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLA7pT4grKKUbQFlrr2iM5W1pfR3-g",
                                                            "width": 246,
                                                            "height": 138
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/26MinSlgHoE/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLCmtkxo_ycKcCqORF-PuWboF6Vhrg",
                                                            "width": 336,
                                                            "height": 188
                                                        }]
                                                    },
                                                    "title": {
                                                        "runs": [{
                                                            "text": "GitHub CodeSpaces和Dev Container系列——关于Git的一些问题"
                                                        }],
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "GitHub CodeSpaces和Dev Container系列——关于Git的一些问题 by 麦兜搞IT 2 months ago 12 minutes, 44 seconds 87 views"
                                                            }
                                                        }
                                                    },
                                                    "descriptionSnippet": {
                                                        "runs": [{
                                                            "text": "视频中使用的链接\nhttps://github.com/xiaopeng163/terraform-dev-container\n\nSharing Git credentials with your container： https://code.visualstudio.com/docs/devcontainers/containers#_sharing-..."
                                                        }]
                                                    },
                                                    "publishedTimeText": {
                                                        "simpleText": "2 months ago"
                                                    },
                                                    "lengthText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "12 minutes, 44 seconds"
                                                            }
                                                        },
                                                        "simpleText": "12:44"
                                                    },
                                                    "viewCountText": {
                                                        "simpleText": "87 views"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CGwQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxaGFVDbWpkaHdNR1N1dDhtWjFDcW5SampVd5oBBhDyOBjgB6oBGlVVTEZtamRod01HU3V0OG1aMUNxblJqalV3",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "url": "/watch?v=26MinSlgHoE",
                                                                "webPageType": "WEB_PAGE_TYPE_WATCH",
                                                                "rootVe": 3832
                                                            }
                                                        },
                                                        "watchEndpoint": {
                                                            "videoId": "26MinSlgHoE",
                                                            "watchEndpointSupportedOnesieConfig": {
                                                                "html5PlaybackOnesieConfig": {
                                                                    "commonConfig": {
                                                                        "url": "https://rr3---sn-cnoa-jv3s.googlevideo.com/initplayback?source=youtube\u0026oeis=1\u0026c=WEB\u0026oad=3200\u0026ovd=3200\u0026oaad=11000\u0026oavd=11000\u0026ocs=700\u0026oewis=1\u0026oputc=1\u0026ofpcc=1\u0026msp=1\u0026odepv=1\u0026id=dba3229d29601e81\u0026ip=117.213.17.165\u0026initcwndbps=977500\u0026mt=1679566621\u0026oweuc=\u0026pxtags=Cg4KAnR4EggyNDQyODQxNQ\u0026rxtags=Cg4KAnR4EggyNDQyODQxNA%2CCg4KAnR4EggyNDQyODQxNQ%2CCg4KAnR4EggyNDQyODQxNg%2CCg4KAnR4EggyNDQyODQxNw%2CCg4KAnR4EggyNDQyODQxOA%2CCg4KAnR4EggyNDQyODQxOQ%2CCg4KAnR4EggyNDQyODQyMA"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CGwQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxAgb2Ay9LTyNHbAaoBGlVVTEZtamRod01HU3V0OG1aMUNxblJqalV3",
                                                    "showActionMenu": false,
                                                    "shortViewCountText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "87 views"
                                                            }
                                                        },
                                                        "simpleText": "87 views"
                                                    },
                                                    "menu": {
                                                        "menuRenderer": {
                                                            "items": [{
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Add to queue"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "ADD_TO_QUEUE_TAIL"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CHAQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true
                                                                            }
                                                                        },
                                                                        "signalServiceEndpoint": {
                                                                            "signal": "CLIENT_SIGNAL",
                                                                            "actions": [{
                                                                                "clickTrackingParams": "CHAQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "addToPlaylistCommand": {
                                                                                    "openMiniplayer": true,
                                                                                    "videoId": "26MinSlgHoE",
                                                                                    "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                                    "onCreateListCommand": {
                                                                                        "clickTrackingParams": "CHAQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                        "commandMetadata": {
                                                                                            "webCommandMetadata": {
                                                                                                "sendPost": true,
                                                                                                "apiUrl": "/youtubei/v1/playlist/create"
                                                                                            }
                                                                                        },
                                                                                        "createPlaylistServiceEndpoint": {
                                                                                            "videoIds": ["26MinSlgHoE"],
                                                                                            "params": "CAQ%3D"
                                                                                        }
                                                                                    },
                                                                                    "videoIds": ["26MinSlgHoE"]
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CHAQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }, {
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Share"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "SHARE"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CGwQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true,
                                                                                "apiUrl": "/youtubei/v1/share/get_share_panel"
                                                                            }
                                                                        },
                                                                        "shareEntityServiceEndpoint": {
                                                                            "serializedShareEntity": "CgsyNk1pblNsZ0hvRQ%3D%3D",
                                                                            "commands": [{
                                                                                "clickTrackingParams": "CGwQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "openPopupAction": {
                                                                                    "popup": {
                                                                                        "unifiedSharePanelRenderer": {
                                                                                            "trackingParams": "CG8QjmIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                            "showLoadingSpinner": true
                                                                                        }
                                                                                    },
                                                                                    "popupType": "DIALOG",
                                                                                    "beReused": true
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CGwQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }],
                                                            "trackingParams": "CGwQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                            "accessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Action menu"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "thumbnailOverlays": [{
                                                        "thumbnailOverlayTimeStatusRenderer": {
                                                            "text": {
                                                                "accessibility": {
                                                                    "accessibilityData": {
                                                                        "label": "12 minutes, 44 seconds"
                                                                    }
                                                                },
                                                                "simpleText": "12:44"
                                                            },
                                                            "style": "DEFAULT"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "isToggled": false,
                                                            "untoggledIcon": {
                                                                "iconType": "WATCH_LATER"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "CHECK"
                                                            },
                                                            "untoggledTooltip": "Watch later",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CG4Q-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "addedVideoId": "26MinSlgHoE",
                                                                        "action": "ACTION_ADD_VIDEO"
                                                                    }]
                                                                }
                                                            },
                                                            "toggledServiceEndpoint": {
                                                                "clickTrackingParams": "CG4Q-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "action": "ACTION_REMOVE_VIDEO_BY_VIDEO_ID",
                                                                        "removedVideoId": "26MinSlgHoE"
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Watch later"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CG4Q-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "untoggledIcon": {
                                                                "iconType": "ADD_TO_QUEUE_TAIL"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "PLAYLIST_ADD_CHECK"
                                                            },
                                                            "untoggledTooltip": "Add to queue",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CG0Qx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true
                                                                    }
                                                                },
                                                                "signalServiceEndpoint": {
                                                                    "signal": "CLIENT_SIGNAL",
                                                                    "actions": [{
                                                                        "clickTrackingParams": "CG0Qx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "addToPlaylistCommand": {
                                                                            "openMiniplayer": true,
                                                                            "videoId": "26MinSlgHoE",
                                                                            "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                            "onCreateListCommand": {
                                                                                "clickTrackingParams": "CG0Qx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "commandMetadata": {
                                                                                    "webCommandMetadata": {
                                                                                        "sendPost": true,
                                                                                        "apiUrl": "/youtubei/v1/playlist/create"
                                                                                    }
                                                                                },
                                                                                "createPlaylistServiceEndpoint": {
                                                                                    "videoIds": ["26MinSlgHoE"],
                                                                                    "params": "CAQ%3D"
                                                                                }
                                                                            },
                                                                            "videoIds": ["26MinSlgHoE"]
                                                                        }
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Add to queue"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CG0Qx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayNowPlayingRenderer": {
                                                            "text": {
                                                                "runs": [{
                                                                    "text": "Now playing"
                                                                }]
                                                            }
                                                        }
                                                    }],
                                                    "richThumbnail": {
                                                        "movingThumbnailRenderer": {
                                                            "movingThumbnailDetails": {
                                                                "thumbnails": [{
                                                                    "url": "https://i.ytimg.com/an_webp/26MinSlgHoE/mqdefault_6s.webp?du=3000\u0026sqp=CLam8KAG\u0026rs=AOn4CLAPLoKVGGfME6Xp-QZVHMX29sydfg",
                                                                    "width": 320,
                                                                    "height": 180
                                                                }],
                                                                "logAsMovingThumbnail": true
                                                            },
                                                            "enableHoveredLogging": true,
                                                            "enableOverlay": true
                                                        }
                                                    }
                                                }
                                            },
                                            "trackingParams": "CGsQmY0FGBAiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                        }
                                    }, {
                                        "richItemRenderer": {
                                            "content": {
                                                "videoRenderer": {
                                                    "videoId": "Z8aLjIRJcnE",
                                                    "thumbnail": {
                                                        "thumbnails": [{
                                                            "url": "https://i.ytimg.com/vi/Z8aLjIRJcnE/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLDlNH7L-tFrvK84jQF4EP8T_7vyfg",
                                                            "width": 168,
                                                            "height": 94
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/Z8aLjIRJcnE/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLDZWr0FN1yvQMYEwY4TM77GHS7jLA",
                                                            "width": 196,
                                                            "height": 110
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/Z8aLjIRJcnE/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLBiaHc6C8XnB6oX4brGyZA-3bqw8A",
                                                            "width": 246,
                                                            "height": 138
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/Z8aLjIRJcnE/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLAouMgGZRtpge_MEd6GTbAD-LiFTg",
                                                            "width": 336,
                                                            "height": 188
                                                        }]
                                                    },
                                                    "title": {
                                                        "runs": [{
                                                            "text": "GitHub CodeSpaces和Dev Container系列——客制化的Dev Container"
                                                        }],
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "GitHub CodeSpaces和Dev Container系列——客制化的Dev Container by 麦兜搞IT 2 months ago 11 minutes, 28 seconds 87 views"
                                                            }
                                                        }
                                                    },
                                                    "descriptionSnippet": {
                                                        "runs": [{
                                                            "text": "视频中使用的链接\nhttps://github.com/xiaopeng163/terraform-dev-container\n\n===============================\n【我的一些在线课程】\n\nUdemy：https://github.com/udemy-course\nHiskio：htt..."
                                                        }]
                                                    },
                                                    "publishedTimeText": {
                                                        "simpleText": "2 months ago"
                                                    },
                                                    "lengthText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "11 minutes, 28 seconds"
                                                            }
                                                        },
                                                        "simpleText": "11:28"
                                                    },
                                                    "viewCountText": {
                                                        "simpleText": "87 views"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CGYQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxaGFVDbWpkaHdNR1N1dDhtWjFDcW5SampVd5oBBhDyOBjgB6oBGlVVTEZtamRod01HU3V0OG1aMUNxblJqalV3",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "url": "/watch?v=Z8aLjIRJcnE",
                                                                "webPageType": "WEB_PAGE_TYPE_WATCH",
                                                                "rootVe": 3832
                                                            }
                                                        },
                                                        "watchEndpoint": {
                                                            "videoId": "Z8aLjIRJcnE",
                                                            "watchEndpointSupportedOnesieConfig": {
                                                                "html5PlaybackOnesieConfig": {
                                                                    "commonConfig": {
                                                                        "url": "https://rr6---sn-cnoa-jv3s.googlevideo.com/initplayback?source=youtube\u0026oeis=1\u0026c=WEB\u0026oad=3200\u0026ovd=3200\u0026oaad=11000\u0026oavd=11000\u0026ocs=700\u0026oewis=1\u0026oputc=1\u0026ofpcc=1\u0026msp=1\u0026odepv=1\u0026id=67c68b8c84497271\u0026ip=117.213.17.165\u0026initcwndbps=977500\u0026mt=1679566621\u0026oweuc=\u0026pxtags=Cg4KAnR4EggyNDQyODQxNQ\u0026rxtags=Cg4KAnR4EggyNDQyODQxNA%2CCg4KAnR4EggyNDQyODQxNQ%2CCg4KAnR4EggyNDQyODQxNg%2CCg4KAnR4EggyNDQyODQxNw%2CCg4KAnR4EggyNDQyODQxOA%2CCg4KAnR4EggyNDQyODQxOQ%2CCg4KAnR4EggyNDQyODQyMA"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CGYQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxA8eSlosjxouNnqgEaVVVMRm1qZGh3TUdTdXQ4bVoxQ3FuUmpqVXc=",
                                                    "showActionMenu": false,
                                                    "shortViewCountText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "87 views"
                                                            }
                                                        },
                                                        "simpleText": "87 views"
                                                    },
                                                    "menu": {
                                                        "menuRenderer": {
                                                            "items": [{
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Add to queue"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "ADD_TO_QUEUE_TAIL"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CGoQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true
                                                                            }
                                                                        },
                                                                        "signalServiceEndpoint": {
                                                                            "signal": "CLIENT_SIGNAL",
                                                                            "actions": [{
                                                                                "clickTrackingParams": "CGoQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "addToPlaylistCommand": {
                                                                                    "openMiniplayer": true,
                                                                                    "videoId": "Z8aLjIRJcnE",
                                                                                    "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                                    "onCreateListCommand": {
                                                                                        "clickTrackingParams": "CGoQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                        "commandMetadata": {
                                                                                            "webCommandMetadata": {
                                                                                                "sendPost": true,
                                                                                                "apiUrl": "/youtubei/v1/playlist/create"
                                                                                            }
                                                                                        },
                                                                                        "createPlaylistServiceEndpoint": {
                                                                                            "videoIds": ["Z8aLjIRJcnE"],
                                                                                            "params": "CAQ%3D"
                                                                                        }
                                                                                    },
                                                                                    "videoIds": ["Z8aLjIRJcnE"]
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CGoQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }, {
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Share"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "SHARE"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CGYQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true,
                                                                                "apiUrl": "/youtubei/v1/share/get_share_panel"
                                                                            }
                                                                        },
                                                                        "shareEntityServiceEndpoint": {
                                                                            "serializedShareEntity": "CgtaOGFMaklSSmNuRQ%3D%3D",
                                                                            "commands": [{
                                                                                "clickTrackingParams": "CGYQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "openPopupAction": {
                                                                                    "popup": {
                                                                                        "unifiedSharePanelRenderer": {
                                                                                            "trackingParams": "CGkQjmIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                            "showLoadingSpinner": true
                                                                                        }
                                                                                    },
                                                                                    "popupType": "DIALOG",
                                                                                    "beReused": true
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CGYQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }],
                                                            "trackingParams": "CGYQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                            "accessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Action menu"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "thumbnailOverlays": [{
                                                        "thumbnailOverlayTimeStatusRenderer": {
                                                            "text": {
                                                                "accessibility": {
                                                                    "accessibilityData": {
                                                                        "label": "11 minutes, 28 seconds"
                                                                    }
                                                                },
                                                                "simpleText": "11:28"
                                                            },
                                                            "style": "DEFAULT"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "isToggled": false,
                                                            "untoggledIcon": {
                                                                "iconType": "WATCH_LATER"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "CHECK"
                                                            },
                                                            "untoggledTooltip": "Watch later",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CGgQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "addedVideoId": "Z8aLjIRJcnE",
                                                                        "action": "ACTION_ADD_VIDEO"
                                                                    }]
                                                                }
                                                            },
                                                            "toggledServiceEndpoint": {
                                                                "clickTrackingParams": "CGgQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "action": "ACTION_REMOVE_VIDEO_BY_VIDEO_ID",
                                                                        "removedVideoId": "Z8aLjIRJcnE"
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Watch later"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CGgQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "untoggledIcon": {
                                                                "iconType": "ADD_TO_QUEUE_TAIL"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "PLAYLIST_ADD_CHECK"
                                                            },
                                                            "untoggledTooltip": "Add to queue",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CGcQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true
                                                                    }
                                                                },
                                                                "signalServiceEndpoint": {
                                                                    "signal": "CLIENT_SIGNAL",
                                                                    "actions": [{
                                                                        "clickTrackingParams": "CGcQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "addToPlaylistCommand": {
                                                                            "openMiniplayer": true,
                                                                            "videoId": "Z8aLjIRJcnE",
                                                                            "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                            "onCreateListCommand": {
                                                                                "clickTrackingParams": "CGcQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "commandMetadata": {
                                                                                    "webCommandMetadata": {
                                                                                        "sendPost": true,
                                                                                        "apiUrl": "/youtubei/v1/playlist/create"
                                                                                    }
                                                                                },
                                                                                "createPlaylistServiceEndpoint": {
                                                                                    "videoIds": ["Z8aLjIRJcnE"],
                                                                                    "params": "CAQ%3D"
                                                                                }
                                                                            },
                                                                            "videoIds": ["Z8aLjIRJcnE"]
                                                                        }
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Add to queue"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CGcQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayNowPlayingRenderer": {
                                                            "text": {
                                                                "runs": [{
                                                                    "text": "Now playing"
                                                                }]
                                                            }
                                                        }
                                                    }],
                                                    "richThumbnail": {
                                                        "movingThumbnailRenderer": {
                                                            "movingThumbnailDetails": {
                                                                "thumbnails": [{
                                                                    "url": "https://i.ytimg.com/an_webp/Z8aLjIRJcnE/mqdefault_6s.webp?du=3000\u0026sqp=CN6z8KAG\u0026rs=AOn4CLBiGrarjwjOfYGqGZu3KTa9u_GpCA",
                                                                    "width": 320,
                                                                    "height": 180
                                                                }],
                                                                "logAsMovingThumbnail": true
                                                            },
                                                            "enableHoveredLogging": true,
                                                            "enableOverlay": true
                                                        }
                                                    }
                                                }
                                            },
                                            "trackingParams": "CGUQmY0FGBEiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                        }
                                    }, {
                                        "richItemRenderer": {
                                            "content": {
                                                "videoRenderer": {
                                                    "videoId": "alCbFl0eX6c",
                                                    "thumbnail": {
                                                        "thumbnails": [{
                                                            "url": "https://i.ytimg.com/vi/alCbFl0eX6c/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLBuzQl2-WWO9-8uReDdQhpLensB5w",
                                                            "width": 168,
                                                            "height": 94
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/alCbFl0eX6c/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLB1MEUZSQPA74pt6Jsivily0YVeYA",
                                                            "width": 196,
                                                            "height": 110
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/alCbFl0eX6c/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLDchTStfZ7mLZrUXMq5nHKYE7rC0Q",
                                                            "width": 246,
                                                            "height": 138
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/alCbFl0eX6c/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLDJm7G8MJ-uliboW3MN9gA0Hs62vg",
                                                            "width": 336,
                                                            "height": 188
                                                        }]
                                                    },
                                                    "title": {
                                                        "runs": [{
                                                            "text": "GitHub CodeSpaces和Dev Container系列——Dev Container的基本使用"
                                                        }],
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "GitHub CodeSpaces和Dev Container系列——Dev Container的基本使用 by 麦兜搞IT 2 months ago 14 minutes, 11 seconds 211 views"
                                                            }
                                                        }
                                                    },
                                                    "descriptionSnippet": {
                                                        "runs": [{
                                                            "text": "视频中使用的链接\nhttps://github.com/xiaopeng163/python-dev-container-demo\nhttps://github.com/features/codespaces\n\n===============================\n【我的一些在线课程】\n\nUdemy：htt..."
                                                        }]
                                                    },
                                                    "publishedTimeText": {
                                                        "simpleText": "2 months ago"
                                                    },
                                                    "lengthText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "14 minutes, 11 seconds"
                                                            }
                                                        },
                                                        "simpleText": "14:11"
                                                    },
                                                    "viewCountText": {
                                                        "simpleText": "211 views"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CGAQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxaGFVDbWpkaHdNR1N1dDhtWjFDcW5SampVd5oBBhDyOBjgB6oBGlVVTEZtamRod01HU3V0OG1aMUNxblJqalV3",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "url": "/watch?v=alCbFl0eX6c",
                                                                "webPageType": "WEB_PAGE_TYPE_WATCH",
                                                                "rootVe": 3832
                                                            }
                                                        },
                                                        "watchEndpoint": {
                                                            "videoId": "alCbFl0eX6c",
                                                            "watchEndpointSupportedOnesieConfig": {
                                                                "html5PlaybackOnesieConfig": {
                                                                    "commonConfig": {
                                                                        "url": "https://rr5---sn-cnoa-jv3s.googlevideo.com/initplayback?source=youtube\u0026oeis=1\u0026c=WEB\u0026oad=3200\u0026ovd=3200\u0026oaad=11000\u0026oavd=11000\u0026ocs=700\u0026oewis=1\u0026oputc=1\u0026ofpcc=1\u0026msp=1\u0026odepv=1\u0026id=6a509b165d1e5fa7\u0026ip=117.213.17.165\u0026initcwndbps=977500\u0026mt=1679566621\u0026oweuc=\u0026pxtags=Cg4KAnR4EggyNDQyODQxNQ\u0026rxtags=Cg4KAnR4EggyNDQyODQxNA%2CCg4KAnR4EggyNDQyODQxNQ%2CCg4KAnR4EggyNDQyODQxNg%2CCg4KAnR4EggyNDQyODQxNw%2CCg4KAnR4EggyNDQyODQxOA%2CCg4KAnR4EggyNDQyODQxOQ%2CCg4KAnR4EggyNDQyODQyMA"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CGAQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxAp7_56OXipqhqqgEaVVVMRm1qZGh3TUdTdXQ4bVoxQ3FuUmpqVXc=",
                                                    "showActionMenu": false,
                                                    "shortViewCountText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "211 views"
                                                            }
                                                        },
                                                        "simpleText": "211 views"
                                                    },
                                                    "menu": {
                                                        "menuRenderer": {
                                                            "items": [{
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Add to queue"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "ADD_TO_QUEUE_TAIL"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CGQQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true
                                                                            }
                                                                        },
                                                                        "signalServiceEndpoint": {
                                                                            "signal": "CLIENT_SIGNAL",
                                                                            "actions": [{
                                                                                "clickTrackingParams": "CGQQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "addToPlaylistCommand": {
                                                                                    "openMiniplayer": true,
                                                                                    "videoId": "alCbFl0eX6c",
                                                                                    "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                                    "onCreateListCommand": {
                                                                                        "clickTrackingParams": "CGQQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                        "commandMetadata": {
                                                                                            "webCommandMetadata": {
                                                                                                "sendPost": true,
                                                                                                "apiUrl": "/youtubei/v1/playlist/create"
                                                                                            }
                                                                                        },
                                                                                        "createPlaylistServiceEndpoint": {
                                                                                            "videoIds": ["alCbFl0eX6c"],
                                                                                            "params": "CAQ%3D"
                                                                                        }
                                                                                    },
                                                                                    "videoIds": ["alCbFl0eX6c"]
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CGQQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }, {
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Share"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "SHARE"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CGAQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true,
                                                                                "apiUrl": "/youtubei/v1/share/get_share_panel"
                                                                            }
                                                                        },
                                                                        "shareEntityServiceEndpoint": {
                                                                            "serializedShareEntity": "CgthbENiRmwwZVg2Yw%3D%3D",
                                                                            "commands": [{
                                                                                "clickTrackingParams": "CGAQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "openPopupAction": {
                                                                                    "popup": {
                                                                                        "unifiedSharePanelRenderer": {
                                                                                            "trackingParams": "CGMQjmIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                            "showLoadingSpinner": true
                                                                                        }
                                                                                    },
                                                                                    "popupType": "DIALOG",
                                                                                    "beReused": true
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CGAQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }],
                                                            "trackingParams": "CGAQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                            "accessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Action menu"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "thumbnailOverlays": [{
                                                        "thumbnailOverlayTimeStatusRenderer": {
                                                            "text": {
                                                                "accessibility": {
                                                                    "accessibilityData": {
                                                                        "label": "14 minutes, 11 seconds"
                                                                    }
                                                                },
                                                                "simpleText": "14:11"
                                                            },
                                                            "style": "DEFAULT"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "isToggled": false,
                                                            "untoggledIcon": {
                                                                "iconType": "WATCH_LATER"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "CHECK"
                                                            },
                                                            "untoggledTooltip": "Watch later",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CGIQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "addedVideoId": "alCbFl0eX6c",
                                                                        "action": "ACTION_ADD_VIDEO"
                                                                    }]
                                                                }
                                                            },
                                                            "toggledServiceEndpoint": {
                                                                "clickTrackingParams": "CGIQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "action": "ACTION_REMOVE_VIDEO_BY_VIDEO_ID",
                                                                        "removedVideoId": "alCbFl0eX6c"
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Watch later"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CGIQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "untoggledIcon": {
                                                                "iconType": "ADD_TO_QUEUE_TAIL"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "PLAYLIST_ADD_CHECK"
                                                            },
                                                            "untoggledTooltip": "Add to queue",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CGEQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true
                                                                    }
                                                                },
                                                                "signalServiceEndpoint": {
                                                                    "signal": "CLIENT_SIGNAL",
                                                                    "actions": [{
                                                                        "clickTrackingParams": "CGEQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "addToPlaylistCommand": {
                                                                            "openMiniplayer": true,
                                                                            "videoId": "alCbFl0eX6c",
                                                                            "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                            "onCreateListCommand": {
                                                                                "clickTrackingParams": "CGEQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "commandMetadata": {
                                                                                    "webCommandMetadata": {
                                                                                        "sendPost": true,
                                                                                        "apiUrl": "/youtubei/v1/playlist/create"
                                                                                    }
                                                                                },
                                                                                "createPlaylistServiceEndpoint": {
                                                                                    "videoIds": ["alCbFl0eX6c"],
                                                                                    "params": "CAQ%3D"
                                                                                }
                                                                            },
                                                                            "videoIds": ["alCbFl0eX6c"]
                                                                        }
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Add to queue"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CGEQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayNowPlayingRenderer": {
                                                            "text": {
                                                                "runs": [{
                                                                    "text": "Now playing"
                                                                }]
                                                            }
                                                        }
                                                    }],
                                                    "richThumbnail": {
                                                        "movingThumbnailRenderer": {
                                                            "movingThumbnailDetails": {
                                                                "thumbnails": [{
                                                                    "url": "https://i.ytimg.com/an_webp/alCbFl0eX6c/mqdefault_6s.webp?du=3000\u0026sqp=CMGd8KAG\u0026rs=AOn4CLCBkSS9xVgfpmDi9WupJJugMCxZeA",
                                                                    "width": 320,
                                                                    "height": 180
                                                                }],
                                                                "logAsMovingThumbnail": true
                                                            },
                                                            "enableHoveredLogging": true,
                                                            "enableOverlay": true
                                                        }
                                                    }
                                                }
                                            },
                                            "trackingParams": "CF8QmY0FGBIiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                        }
                                    }, {
                                        "richItemRenderer": {
                                            "content": {
                                                "videoRenderer": {
                                                    "videoId": "L6LcW22CB6A",
                                                    "thumbnail": {
                                                        "thumbnails": [{
                                                            "url": "https://i.ytimg.com/vi/L6LcW22CB6A/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLASz7tKm0z17DrwA4m1KOKgY64DPA",
                                                            "width": 168,
                                                            "height": 94
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/L6LcW22CB6A/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLDybLHPrTc4MiKXvEhven6_5QlzJA",
                                                            "width": 196,
                                                            "height": 110
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/L6LcW22CB6A/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLBAk4eir2aabBaybIpjSChwA-lZ0w",
                                                            "width": 246,
                                                            "height": 138
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/L6LcW22CB6A/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLCENu_gu9gXxnI61c407kE4NVkTKg",
                                                            "width": 336,
                                                            "height": 188
                                                        }]
                                                    },
                                                    "title": {
                                                        "runs": [{
                                                            "text": "GitHub CodeSpaces和Dev Container系列——Codespace初体验"
                                                        }],
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "GitHub CodeSpaces和Dev Container系列——Codespace初体验 by 麦兜搞IT 3 months ago 10 minutes, 24 seconds 683 views"
                                                            }
                                                        }
                                                    },
                                                    "descriptionSnippet": {
                                                        "runs": [{
                                                            "text": "视频中使用的链接\nhttps://github.com/xiaopeng163/python-dev-container-demo\nhttps://github.com/features/codespaces\n\n===============================\n【我的一些在线课程】\n\nUdemy：htt..."
                                                        }]
                                                    },
                                                    "publishedTimeText": {
                                                        "simpleText": "3 months ago"
                                                    },
                                                    "lengthText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "10 minutes, 24 seconds"
                                                            }
                                                        },
                                                        "simpleText": "10:24"
                                                    },
                                                    "viewCountText": {
                                                        "simpleText": "683 views"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CFoQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxaGFVDbWpkaHdNR1N1dDhtWjFDcW5SampVd5oBBhDyOBjgB6oBGlVVTEZtamRod01HU3V0OG1aMUNxblJqalV3",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "url": "/watch?v=L6LcW22CB6A",
                                                                "webPageType": "WEB_PAGE_TYPE_WATCH",
                                                                "rootVe": 3832
                                                            }
                                                        },
                                                        "watchEndpoint": {
                                                            "videoId": "L6LcW22CB6A",
                                                            "watchEndpointSupportedOnesieConfig": {
                                                                "html5PlaybackOnesieConfig": {
                                                                    "commonConfig": {
                                                                        "url": "https://rr1---sn-cnoa-jv3l.googlevideo.com/initplayback?source=youtube\u0026oeis=1\u0026c=WEB\u0026oad=3200\u0026ovd=3200\u0026oaad=11000\u0026oavd=11000\u0026ocs=700\u0026oewis=1\u0026oputc=1\u0026ofpcc=1\u0026msp=1\u0026odepv=1\u0026id=2fa2dc5b6d8207a0\u0026ip=117.213.17.165\u0026initcwndbps=1068750\u0026mt=1679566621\u0026oweuc=\u0026pxtags=Cg4KAnR4EggyNDQyODQxNQ\u0026rxtags=Cg4KAnR4EggyNDQyODQxNA%2CCg4KAnR4EggyNDQyODQxNQ%2CCg4KAnR4EggyNDQyODQxNg%2CCg4KAnR4EggyNDQyODQxNw%2CCg4KAnR4EggyNDQyODQxOA%2CCg4KAnR4EggyNDQyODQxOQ%2CCg4KAnR4EggyNDQyODQyMA"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CFoQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxAoI-I7LaLt9EvqgEaVVVMRm1qZGh3TUdTdXQ4bVoxQ3FuUmpqVXc=",
                                                    "showActionMenu": false,
                                                    "shortViewCountText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "683 views"
                                                            }
                                                        },
                                                        "simpleText": "683 views"
                                                    },
                                                    "menu": {
                                                        "menuRenderer": {
                                                            "items": [{
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Add to queue"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "ADD_TO_QUEUE_TAIL"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CF4Q_pgEGAUiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true
                                                                            }
                                                                        },
                                                                        "signalServiceEndpoint": {
                                                                            "signal": "CLIENT_SIGNAL",
                                                                            "actions": [{
                                                                                "clickTrackingParams": "CF4Q_pgEGAUiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "addToPlaylistCommand": {
                                                                                    "openMiniplayer": true,
                                                                                    "videoId": "L6LcW22CB6A",
                                                                                    "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                                    "onCreateListCommand": {
                                                                                        "clickTrackingParams": "CF4Q_pgEGAUiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                        "commandMetadata": {
                                                                                            "webCommandMetadata": {
                                                                                                "sendPost": true,
                                                                                                "apiUrl": "/youtubei/v1/playlist/create"
                                                                                            }
                                                                                        },
                                                                                        "createPlaylistServiceEndpoint": {
                                                                                            "videoIds": ["L6LcW22CB6A"],
                                                                                            "params": "CAQ%3D"
                                                                                        }
                                                                                    },
                                                                                    "videoIds": ["L6LcW22CB6A"]
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CF4Q_pgEGAUiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }, {
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Share"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "SHARE"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CFoQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true,
                                                                                "apiUrl": "/youtubei/v1/share/get_share_panel"
                                                                            }
                                                                        },
                                                                        "shareEntityServiceEndpoint": {
                                                                            "serializedShareEntity": "CgtMNkxjVzIyQ0I2QQ%3D%3D",
                                                                            "commands": [{
                                                                                "clickTrackingParams": "CFoQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "openPopupAction": {
                                                                                    "popup": {
                                                                                        "unifiedSharePanelRenderer": {
                                                                                            "trackingParams": "CF0QjmIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                            "showLoadingSpinner": true
                                                                                        }
                                                                                    },
                                                                                    "popupType": "DIALOG",
                                                                                    "beReused": true
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CFoQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }],
                                                            "trackingParams": "CFoQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                            "accessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Action menu"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "thumbnailOverlays": [{
                                                        "thumbnailOverlayTimeStatusRenderer": {
                                                            "text": {
                                                                "accessibility": {
                                                                    "accessibilityData": {
                                                                        "label": "10 minutes, 24 seconds"
                                                                    }
                                                                },
                                                                "simpleText": "10:24"
                                                            },
                                                            "style": "DEFAULT"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "isToggled": false,
                                                            "untoggledIcon": {
                                                                "iconType": "WATCH_LATER"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "CHECK"
                                                            },
                                                            "untoggledTooltip": "Watch later",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CFwQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "addedVideoId": "L6LcW22CB6A",
                                                                        "action": "ACTION_ADD_VIDEO"
                                                                    }]
                                                                }
                                                            },
                                                            "toggledServiceEndpoint": {
                                                                "clickTrackingParams": "CFwQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "action": "ACTION_REMOVE_VIDEO_BY_VIDEO_ID",
                                                                        "removedVideoId": "L6LcW22CB6A"
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Watch later"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CFwQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "untoggledIcon": {
                                                                "iconType": "ADD_TO_QUEUE_TAIL"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "PLAYLIST_ADD_CHECK"
                                                            },
                                                            "untoggledTooltip": "Add to queue",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CFsQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true
                                                                    }
                                                                },
                                                                "signalServiceEndpoint": {
                                                                    "signal": "CLIENT_SIGNAL",
                                                                    "actions": [{
                                                                        "clickTrackingParams": "CFsQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "addToPlaylistCommand": {
                                                                            "openMiniplayer": true,
                                                                            "videoId": "L6LcW22CB6A",
                                                                            "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                            "onCreateListCommand": {
                                                                                "clickTrackingParams": "CFsQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "commandMetadata": {
                                                                                    "webCommandMetadata": {
                                                                                        "sendPost": true,
                                                                                        "apiUrl": "/youtubei/v1/playlist/create"
                                                                                    }
                                                                                },
                                                                                "createPlaylistServiceEndpoint": {
                                                                                    "videoIds": ["L6LcW22CB6A"],
                                                                                    "params": "CAQ%3D"
                                                                                }
                                                                            },
                                                                            "videoIds": ["L6LcW22CB6A"]
                                                                        }
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Add to queue"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CFsQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayNowPlayingRenderer": {
                                                            "text": {
                                                                "runs": [{
                                                                    "text": "Now playing"
                                                                }]
                                                            }
                                                        }
                                                    }]
                                                }
                                            },
                                            "trackingParams": "CFkQmY0FGBMiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                        }
                                    }, {
                                        "richItemRenderer": {
                                            "content": {
                                                "videoRenderer": {
                                                    "videoId": "ArJFcOZbaCU",
                                                    "thumbnail": {
                                                        "thumbnails": [{
                                                            "url": "https://i.ytimg.com/vi/ArJFcOZbaCU/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLCjDg3TDI6PAGQ2esTXWXTA-_g2YQ",
                                                            "width": 168,
                                                            "height": 94
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/ArJFcOZbaCU/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLDRvMkyujN3roX534W02kSMIgkVFQ",
                                                            "width": 196,
                                                            "height": 110
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/ArJFcOZbaCU/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLCqFGR0uSlkVQZpwC9ZlhKX9_ASZA",
                                                            "width": 246,
                                                            "height": 138
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/ArJFcOZbaCU/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLBBmW9HOcq6rS1-mqbTHvByOFOpGg",
                                                            "width": 336,
                                                            "height": 188
                                                        }]
                                                    },
                                                    "title": {
                                                        "runs": [{
                                                            "text": "Docker构建多架构支持镜像 | docker buildx命令的使用"
                                                        }],
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "Docker构建多架构支持镜像 | docker buildx命令的使用 by 麦兜搞IT 4 months ago 10 minutes, 59 seconds 321 views"
                                                            }
                                                        }
                                                    },
                                                    "descriptionSnippet": {
                                                        "runs": [{
                                                            "text": "文档链接 https://dockertips.readthedocs.io/en/latest/docker-arch.html\n课程购买链接 https://www.udemy.com/course/docker-china/?referralCode=317AFE518BCFEF7FC897\n\n==========================..."
                                                        }]
                                                    },
                                                    "publishedTimeText": {
                                                        "simpleText": "4 months ago"
                                                    },
                                                    "lengthText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "10 minutes, 59 seconds"
                                                            }
                                                        },
                                                        "simpleText": "10:59"
                                                    },
                                                    "viewCountText": {
                                                        "simpleText": "321 views"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CFQQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxaGFVDbWpkaHdNR1N1dDhtWjFDcW5SampVd5oBBhDyOBjgB6oBGlVVTEZtamRod01HU3V0OG1aMUNxblJqalV3",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "url": "/watch?v=ArJFcOZbaCU",
                                                                "webPageType": "WEB_PAGE_TYPE_WATCH",
                                                                "rootVe": 3832
                                                            }
                                                        },
                                                        "watchEndpoint": {
                                                            "videoId": "ArJFcOZbaCU",
                                                            "watchEndpointSupportedOnesieConfig": {
                                                                "html5PlaybackOnesieConfig": {
                                                                    "commonConfig": {
                                                                        "url": "https://rr6---sn-cnoa-jv3s.googlevideo.com/initplayback?source=youtube\u0026oeis=1\u0026c=WEB\u0026oad=3200\u0026ovd=3200\u0026oaad=11000\u0026oavd=11000\u0026ocs=700\u0026oewis=1\u0026oputc=1\u0026ofpcc=1\u0026msp=1\u0026odepv=1\u0026id=02b24570e65b6825\u0026ip=117.213.17.165\u0026initcwndbps=977500\u0026mt=1679566621\u0026oweuc=\u0026pxtags=Cg4KAnR4EggyNDQyODQxNQ\u0026rxtags=Cg4KAnR4EggyNDQyODQxNA%2CCg4KAnR4EggyNDQyODQxNQ%2CCg4KAnR4EggyNDQyODQxNg%2CCg4KAnR4EggyNDQyODQxNw%2CCg4KAnR4EggyNDQyODQxOA%2CCg4KAnR4EggyNDQyODQxOQ%2CCg4KAnR4EggyNDQyODQyMA"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CFQQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxApdDtso6ukdkCqgEaVVVMRm1qZGh3TUdTdXQ4bVoxQ3FuUmpqVXc=",
                                                    "showActionMenu": false,
                                                    "shortViewCountText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "321 views"
                                                            }
                                                        },
                                                        "simpleText": "321 views"
                                                    },
                                                    "menu": {
                                                        "menuRenderer": {
                                                            "items": [{
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Add to queue"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "ADD_TO_QUEUE_TAIL"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CFgQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true
                                                                            }
                                                                        },
                                                                        "signalServiceEndpoint": {
                                                                            "signal": "CLIENT_SIGNAL",
                                                                            "actions": [{
                                                                                "clickTrackingParams": "CFgQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "addToPlaylistCommand": {
                                                                                    "openMiniplayer": true,
                                                                                    "videoId": "ArJFcOZbaCU",
                                                                                    "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                                    "onCreateListCommand": {
                                                                                        "clickTrackingParams": "CFgQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                        "commandMetadata": {
                                                                                            "webCommandMetadata": {
                                                                                                "sendPost": true,
                                                                                                "apiUrl": "/youtubei/v1/playlist/create"
                                                                                            }
                                                                                        },
                                                                                        "createPlaylistServiceEndpoint": {
                                                                                            "videoIds": ["ArJFcOZbaCU"],
                                                                                            "params": "CAQ%3D"
                                                                                        }
                                                                                    },
                                                                                    "videoIds": ["ArJFcOZbaCU"]
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CFgQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }, {
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Share"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "SHARE"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CFQQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true,
                                                                                "apiUrl": "/youtubei/v1/share/get_share_panel"
                                                                            }
                                                                        },
                                                                        "shareEntityServiceEndpoint": {
                                                                            "serializedShareEntity": "CgtBckpGY09aYmFDVQ%3D%3D",
                                                                            "commands": [{
                                                                                "clickTrackingParams": "CFQQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "openPopupAction": {
                                                                                    "popup": {
                                                                                        "unifiedSharePanelRenderer": {
                                                                                            "trackingParams": "CFcQjmIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                            "showLoadingSpinner": true
                                                                                        }
                                                                                    },
                                                                                    "popupType": "DIALOG",
                                                                                    "beReused": true
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CFQQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }],
                                                            "trackingParams": "CFQQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                            "accessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Action menu"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "thumbnailOverlays": [{
                                                        "thumbnailOverlayTimeStatusRenderer": {
                                                            "text": {
                                                                "accessibility": {
                                                                    "accessibilityData": {
                                                                        "label": "10 minutes, 59 seconds"
                                                                    }
                                                                },
                                                                "simpleText": "10:59"
                                                            },
                                                            "style": "DEFAULT"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "isToggled": false,
                                                            "untoggledIcon": {
                                                                "iconType": "WATCH_LATER"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "CHECK"
                                                            },
                                                            "untoggledTooltip": "Watch later",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CFYQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "addedVideoId": "ArJFcOZbaCU",
                                                                        "action": "ACTION_ADD_VIDEO"
                                                                    }]
                                                                }
                                                            },
                                                            "toggledServiceEndpoint": {
                                                                "clickTrackingParams": "CFYQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "action": "ACTION_REMOVE_VIDEO_BY_VIDEO_ID",
                                                                        "removedVideoId": "ArJFcOZbaCU"
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Watch later"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CFYQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "untoggledIcon": {
                                                                "iconType": "ADD_TO_QUEUE_TAIL"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "PLAYLIST_ADD_CHECK"
                                                            },
                                                            "untoggledTooltip": "Add to queue",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CFUQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true
                                                                    }
                                                                },
                                                                "signalServiceEndpoint": {
                                                                    "signal": "CLIENT_SIGNAL",
                                                                    "actions": [{
                                                                        "clickTrackingParams": "CFUQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "addToPlaylistCommand": {
                                                                            "openMiniplayer": true,
                                                                            "videoId": "ArJFcOZbaCU",
                                                                            "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                            "onCreateListCommand": {
                                                                                "clickTrackingParams": "CFUQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "commandMetadata": {
                                                                                    "webCommandMetadata": {
                                                                                        "sendPost": true,
                                                                                        "apiUrl": "/youtubei/v1/playlist/create"
                                                                                    }
                                                                                },
                                                                                "createPlaylistServiceEndpoint": {
                                                                                    "videoIds": ["ArJFcOZbaCU"],
                                                                                    "params": "CAQ%3D"
                                                                                }
                                                                            },
                                                                            "videoIds": ["ArJFcOZbaCU"]
                                                                        }
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Add to queue"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CFUQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayNowPlayingRenderer": {
                                                            "text": {
                                                                "runs": [{
                                                                    "text": "Now playing"
                                                                }]
                                                            }
                                                        }
                                                    }],
                                                    "richThumbnail": {
                                                        "movingThumbnailRenderer": {
                                                            "movingThumbnailDetails": {
                                                                "thumbnails": [{
                                                                    "url": "https://i.ytimg.com/an_webp/ArJFcOZbaCU/mqdefault_6s.webp?du=3000\u0026sqp=CIq98KAG\u0026rs=AOn4CLDU2Uiye1Bc6Iovse6wcbmW6G5w7Q",
                                                                    "width": 320,
                                                                    "height": 180
                                                                }],
                                                                "logAsMovingThumbnail": true
                                                            },
                                                            "enableHoveredLogging": true,
                                                            "enableOverlay": true
                                                        }
                                                    }
                                                }
                                            },
                                            "trackingParams": "CFMQmY0FGBQiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                        }
                                    }, {
                                        "richItemRenderer": {
                                            "content": {
                                                "videoRenderer": {
                                                    "videoId": "JHXNT-CE-tQ",
                                                    "thumbnail": {
                                                        "thumbnails": [{
                                                            "url": "https://i.ytimg.com/vi/JHXNT-CE-tQ/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLBXaFFtmstAxBxE5bI4sJOniaXvVw",
                                                            "width": 168,
                                                            "height": 94
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/JHXNT-CE-tQ/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLDo2JmUlShqpkPwX96ifDxXITVpsQ",
                                                            "width": 196,
                                                            "height": 110
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/JHXNT-CE-tQ/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLACSFZyj9wwPndkx0EPJ4RAN0jXsg",
                                                            "width": 246,
                                                            "height": 138
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/JHXNT-CE-tQ/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLB_BhMcR6CHT_towmJjQJbZuqp_YA",
                                                            "width": 336,
                                                            "height": 188
                                                        }]
                                                    },
                                                    "title": {
                                                        "runs": [{
                                                            "text": "lima和vscode一起使用效果更佳"
                                                        }],
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "lima和vscode一起使用效果更佳 by 麦兜搞IT 5 months ago 5 minutes, 40 seconds 168 views"
                                                            }
                                                        }
                                                    },
                                                    "descriptionSnippet": {
                                                        "runs": [{
                                                            "text": "===============================\n【我的一些在线课程】\n\nUdemy：https://github.com/udemy-course\nHiskio：https://hiskio.com/search?word=peng%20xiao\u0026sort=attendance\u0026page=1\n\n【联系方式..."
                                                        }]
                                                    },
                                                    "publishedTimeText": {
                                                        "simpleText": "5 months ago"
                                                    },
                                                    "lengthText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "5 minutes, 40 seconds"
                                                            }
                                                        },
                                                        "simpleText": "5:40"
                                                    },
                                                    "viewCountText": {
                                                        "simpleText": "168 views"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CE4Q3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxaGFVDbWpkaHdNR1N1dDhtWjFDcW5SampVd5oBBhDyOBjgB6oBGlVVTEZtamRod01HU3V0OG1aMUNxblJqalV3",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "url": "/watch?v=JHXNT-CE-tQ",
                                                                "webPageType": "WEB_PAGE_TYPE_WATCH",
                                                                "rootVe": 3832
                                                            }
                                                        },
                                                        "watchEndpoint": {
                                                            "videoId": "JHXNT-CE-tQ",
                                                            "watchEndpointSupportedOnesieConfig": {
                                                                "html5PlaybackOnesieConfig": {
                                                                    "commonConfig": {
                                                                        "url": "https://rr2---sn-cnoa-jv3s.googlevideo.com/initplayback?source=youtube\u0026oeis=1\u0026c=WEB\u0026oad=3200\u0026ovd=3200\u0026oaad=11000\u0026oavd=11000\u0026ocs=700\u0026oewis=1\u0026oputc=1\u0026ofpcc=1\u0026msp=1\u0026odepv=1\u0026id=2475cd4fe084fad4\u0026ip=117.213.17.165\u0026initcwndbps=977500\u0026mt=1679566621\u0026oweuc=\u0026pxtags=Cg4KAnR4EggyNDQyODQxNQ\u0026rxtags=Cg4KAnR4EggyNDQyODQxNA%2CCg4KAnR4EggyNDQyODQxNQ%2CCg4KAnR4EggyNDQyODQxNg%2CCg4KAnR4EggyNDQyODQxNw%2CCg4KAnR4EggyNDQyODQxOA%2CCg4KAnR4EggyNDQyODQxOQ%2CCg4KAnR4EggyNDQyODQyMA"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CE4Q3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxA1PWThP6p87okqgEaVVVMRm1qZGh3TUdTdXQ4bVoxQ3FuUmpqVXc=",
                                                    "showActionMenu": false,
                                                    "shortViewCountText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "168 views"
                                                            }
                                                        },
                                                        "simpleText": "168 views"
                                                    },
                                                    "menu": {
                                                        "menuRenderer": {
                                                            "items": [{
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Add to queue"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "ADD_TO_QUEUE_TAIL"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CFIQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true
                                                                            }
                                                                        },
                                                                        "signalServiceEndpoint": {
                                                                            "signal": "CLIENT_SIGNAL",
                                                                            "actions": [{
                                                                                "clickTrackingParams": "CFIQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "addToPlaylistCommand": {
                                                                                    "openMiniplayer": true,
                                                                                    "videoId": "JHXNT-CE-tQ",
                                                                                    "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                                    "onCreateListCommand": {
                                                                                        "clickTrackingParams": "CFIQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                        "commandMetadata": {
                                                                                            "webCommandMetadata": {
                                                                                                "sendPost": true,
                                                                                                "apiUrl": "/youtubei/v1/playlist/create"
                                                                                            }
                                                                                        },
                                                                                        "createPlaylistServiceEndpoint": {
                                                                                            "videoIds": ["JHXNT-CE-tQ"],
                                                                                            "params": "CAQ%3D"
                                                                                        }
                                                                                    },
                                                                                    "videoIds": ["JHXNT-CE-tQ"]
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CFIQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }, {
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Share"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "SHARE"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CE4Q3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true,
                                                                                "apiUrl": "/youtubei/v1/share/get_share_panel"
                                                                            }
                                                                        },
                                                                        "shareEntityServiceEndpoint": {
                                                                            "serializedShareEntity": "CgtKSFhOVC1DRS10UQ%3D%3D",
                                                                            "commands": [{
                                                                                "clickTrackingParams": "CE4Q3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "openPopupAction": {
                                                                                    "popup": {
                                                                                        "unifiedSharePanelRenderer": {
                                                                                            "trackingParams": "CFEQjmIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                            "showLoadingSpinner": true
                                                                                        }
                                                                                    },
                                                                                    "popupType": "DIALOG",
                                                                                    "beReused": true
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CE4Q3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }],
                                                            "trackingParams": "CE4Q3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                            "accessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Action menu"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "thumbnailOverlays": [{
                                                        "thumbnailOverlayTimeStatusRenderer": {
                                                            "text": {
                                                                "accessibility": {
                                                                    "accessibilityData": {
                                                                        "label": "5 minutes, 40 seconds"
                                                                    }
                                                                },
                                                                "simpleText": "5:40"
                                                            },
                                                            "style": "DEFAULT"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "isToggled": false,
                                                            "untoggledIcon": {
                                                                "iconType": "WATCH_LATER"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "CHECK"
                                                            },
                                                            "untoggledTooltip": "Watch later",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CFAQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "addedVideoId": "JHXNT-CE-tQ",
                                                                        "action": "ACTION_ADD_VIDEO"
                                                                    }]
                                                                }
                                                            },
                                                            "toggledServiceEndpoint": {
                                                                "clickTrackingParams": "CFAQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "action": "ACTION_REMOVE_VIDEO_BY_VIDEO_ID",
                                                                        "removedVideoId": "JHXNT-CE-tQ"
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Watch later"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CFAQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "untoggledIcon": {
                                                                "iconType": "ADD_TO_QUEUE_TAIL"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "PLAYLIST_ADD_CHECK"
                                                            },
                                                            "untoggledTooltip": "Add to queue",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CE8Qx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true
                                                                    }
                                                                },
                                                                "signalServiceEndpoint": {
                                                                    "signal": "CLIENT_SIGNAL",
                                                                    "actions": [{
                                                                        "clickTrackingParams": "CE8Qx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "addToPlaylistCommand": {
                                                                            "openMiniplayer": true,
                                                                            "videoId": "JHXNT-CE-tQ",
                                                                            "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                            "onCreateListCommand": {
                                                                                "clickTrackingParams": "CE8Qx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "commandMetadata": {
                                                                                    "webCommandMetadata": {
                                                                                        "sendPost": true,
                                                                                        "apiUrl": "/youtubei/v1/playlist/create"
                                                                                    }
                                                                                },
                                                                                "createPlaylistServiceEndpoint": {
                                                                                    "videoIds": ["JHXNT-CE-tQ"],
                                                                                    "params": "CAQ%3D"
                                                                                }
                                                                            },
                                                                            "videoIds": ["JHXNT-CE-tQ"]
                                                                        }
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Add to queue"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CE8Qx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayNowPlayingRenderer": {
                                                            "text": {
                                                                "runs": [{
                                                                    "text": "Now playing"
                                                                }]
                                                            }
                                                        }
                                                    }],
                                                    "richThumbnail": {
                                                        "movingThumbnailRenderer": {
                                                            "movingThumbnailDetails": {
                                                                "thumbnails": [{
                                                                    "url": "https://i.ytimg.com/an_webp/JHXNT-CE-tQ/mqdefault_6s.webp?du=3000\u0026sqp=CLzH8KAG\u0026rs=AOn4CLALaLQ0kSIXrS1hl9UEkLaJieV7BA",
                                                                    "width": 320,
                                                                    "height": 180
                                                                }],
                                                                "logAsMovingThumbnail": true
                                                            },
                                                            "enableHoveredLogging": true,
                                                            "enableOverlay": true
                                                        }
                                                    }
                                                }
                                            },
                                            "trackingParams": "CE0QmY0FGBUiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                        }
                                    }, {
                                        "richItemRenderer": {
                                            "content": {
                                                "videoRenderer": {
                                                    "videoId": "aV4l85XHFGA",
                                                    "thumbnail": {
                                                        "thumbnails": [{
                                                            "url": "https://i.ytimg.com/vi/aV4l85XHFGA/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLD_ONyCB9f17W5e1Upub2ZjM9v4EA",
                                                            "width": 168,
                                                            "height": 94
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/aV4l85XHFGA/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLCdVNgr7QODQH8oWjL0Gqql1qJ9Lg",
                                                            "width": 196,
                                                            "height": 110
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/aV4l85XHFGA/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLA82-nDxSBtcuBPknTx_xEBsSMJew",
                                                            "width": 246,
                                                            "height": 138
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/aV4l85XHFGA/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLCpeBVkkHuYIx4NIOfSzPgkWuqt1A",
                                                            "width": 336,
                                                            "height": 188
                                                        }]
                                                    },
                                                    "title": {
                                                        "runs": [{
                                                            "text": "lima介绍 | Mac M系列 如何玩虚拟机"
                                                        }],
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "lima介绍 | Mac M系列 如何玩虚拟机 by 麦兜搞IT 5 months ago 13 minutes, 23 seconds 288 views"
                                                            }
                                                        }
                                                    },
                                                    "descriptionSnippet": {
                                                        "runs": [{
                                                            "text": "Lima https://github.com/lima-vm/lima\n\n===============================\n【我的一些在线课程】\n\nUdemy：https://github.com/udemy-course\nHiskio：https://hiskio.com/search?word=peng%20xiao\u0026sor..."
                                                        }]
                                                    },
                                                    "publishedTimeText": {
                                                        "simpleText": "5 months ago"
                                                    },
                                                    "lengthText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "13 minutes, 23 seconds"
                                                            }
                                                        },
                                                        "simpleText": "13:23"
                                                    },
                                                    "viewCountText": {
                                                        "simpleText": "288 views"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CEgQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxaGFVDbWpkaHdNR1N1dDhtWjFDcW5SampVd5oBBhDyOBjgB6oBGlVVTEZtamRod01HU3V0OG1aMUNxblJqalV3",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "url": "/watch?v=aV4l85XHFGA",
                                                                "webPageType": "WEB_PAGE_TYPE_WATCH",
                                                                "rootVe": 3832
                                                            }
                                                        },
                                                        "watchEndpoint": {
                                                            "videoId": "aV4l85XHFGA",
                                                            "watchEndpointSupportedOnesieConfig": {
                                                                "html5PlaybackOnesieConfig": {
                                                                    "commonConfig": {
                                                                        "url": "https://rr3---sn-cnoa-jv3l.googlevideo.com/initplayback?source=youtube\u0026oeis=1\u0026c=WEB\u0026oad=3200\u0026ovd=3200\u0026oaad=11000\u0026oavd=11000\u0026ocs=700\u0026oewis=1\u0026oputc=1\u0026ofpcc=1\u0026msp=1\u0026odepv=1\u0026id=695e25f395c71460\u0026ip=117.213.17.165\u0026initcwndbps=1068750\u0026mt=1679566621\u0026oweuc=\u0026pxtags=Cg4KAnR4EggyNDQyODQxNQ\u0026rxtags=Cg4KAnR4EggyNDQyODQxNA%2CCg4KAnR4EggyNDQyODQxNQ%2CCg4KAnR4EggyNDQyODQxNg%2CCg4KAnR4EggyNDQyODQxNw%2CCg4KAnR4EggyNDQyODQxOA%2CCg4KAnR4EggyNDQyODQxOQ%2CCg4KAnR4EggyNDQyODQyMA"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CEgQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxA4Kicrrm-ia9pqgEaVVVMRm1qZGh3TUdTdXQ4bVoxQ3FuUmpqVXc=",
                                                    "showActionMenu": false,
                                                    "shortViewCountText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "288 views"
                                                            }
                                                        },
                                                        "simpleText": "288 views"
                                                    },
                                                    "menu": {
                                                        "menuRenderer": {
                                                            "items": [{
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Add to queue"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "ADD_TO_QUEUE_TAIL"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CEwQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true
                                                                            }
                                                                        },
                                                                        "signalServiceEndpoint": {
                                                                            "signal": "CLIENT_SIGNAL",
                                                                            "actions": [{
                                                                                "clickTrackingParams": "CEwQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "addToPlaylistCommand": {
                                                                                    "openMiniplayer": true,
                                                                                    "videoId": "aV4l85XHFGA",
                                                                                    "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                                    "onCreateListCommand": {
                                                                                        "clickTrackingParams": "CEwQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                        "commandMetadata": {
                                                                                            "webCommandMetadata": {
                                                                                                "sendPost": true,
                                                                                                "apiUrl": "/youtubei/v1/playlist/create"
                                                                                            }
                                                                                        },
                                                                                        "createPlaylistServiceEndpoint": {
                                                                                            "videoIds": ["aV4l85XHFGA"],
                                                                                            "params": "CAQ%3D"
                                                                                        }
                                                                                    },
                                                                                    "videoIds": ["aV4l85XHFGA"]
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CEwQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }, {
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Share"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "SHARE"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CEgQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true,
                                                                                "apiUrl": "/youtubei/v1/share/get_share_panel"
                                                                            }
                                                                        },
                                                                        "shareEntityServiceEndpoint": {
                                                                            "serializedShareEntity": "CgthVjRsODVYSEZHQQ%3D%3D",
                                                                            "commands": [{
                                                                                "clickTrackingParams": "CEgQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "openPopupAction": {
                                                                                    "popup": {
                                                                                        "unifiedSharePanelRenderer": {
                                                                                            "trackingParams": "CEsQjmIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                            "showLoadingSpinner": true
                                                                                        }
                                                                                    },
                                                                                    "popupType": "DIALOG",
                                                                                    "beReused": true
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CEgQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }],
                                                            "trackingParams": "CEgQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                            "accessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Action menu"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "thumbnailOverlays": [{
                                                        "thumbnailOverlayTimeStatusRenderer": {
                                                            "text": {
                                                                "accessibility": {
                                                                    "accessibilityData": {
                                                                        "label": "13 minutes, 23 seconds"
                                                                    }
                                                                },
                                                                "simpleText": "13:23"
                                                            },
                                                            "style": "DEFAULT"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "isToggled": false,
                                                            "untoggledIcon": {
                                                                "iconType": "WATCH_LATER"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "CHECK"
                                                            },
                                                            "untoggledTooltip": "Watch later",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CEoQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "addedVideoId": "aV4l85XHFGA",
                                                                        "action": "ACTION_ADD_VIDEO"
                                                                    }]
                                                                }
                                                            },
                                                            "toggledServiceEndpoint": {
                                                                "clickTrackingParams": "CEoQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "action": "ACTION_REMOVE_VIDEO_BY_VIDEO_ID",
                                                                        "removedVideoId": "aV4l85XHFGA"
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Watch later"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CEoQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "untoggledIcon": {
                                                                "iconType": "ADD_TO_QUEUE_TAIL"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "PLAYLIST_ADD_CHECK"
                                                            },
                                                            "untoggledTooltip": "Add to queue",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CEkQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true
                                                                    }
                                                                },
                                                                "signalServiceEndpoint": {
                                                                    "signal": "CLIENT_SIGNAL",
                                                                    "actions": [{
                                                                        "clickTrackingParams": "CEkQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "addToPlaylistCommand": {
                                                                            "openMiniplayer": true,
                                                                            "videoId": "aV4l85XHFGA",
                                                                            "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                            "onCreateListCommand": {
                                                                                "clickTrackingParams": "CEkQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "commandMetadata": {
                                                                                    "webCommandMetadata": {
                                                                                        "sendPost": true,
                                                                                        "apiUrl": "/youtubei/v1/playlist/create"
                                                                                    }
                                                                                },
                                                                                "createPlaylistServiceEndpoint": {
                                                                                    "videoIds": ["aV4l85XHFGA"],
                                                                                    "params": "CAQ%3D"
                                                                                }
                                                                            },
                                                                            "videoIds": ["aV4l85XHFGA"]
                                                                        }
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Add to queue"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CEkQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayNowPlayingRenderer": {
                                                            "text": {
                                                                "runs": [{
                                                                    "text": "Now playing"
                                                                }]
                                                            }
                                                        }
                                                    }],
                                                    "richThumbnail": {
                                                        "movingThumbnailRenderer": {
                                                            "movingThumbnailDetails": {
                                                                "thumbnails": [{
                                                                    "url": "https://i.ytimg.com/an_webp/aV4l85XHFGA/mqdefault_6s.webp?du=3000\u0026sqp=CICw8KAG\u0026rs=AOn4CLAy7uaN_aB2umflRmFb7DB1mwySyQ",
                                                                    "width": 320,
                                                                    "height": 180
                                                                }],
                                                                "logAsMovingThumbnail": true
                                                            },
                                                            "enableHoveredLogging": true,
                                                            "enableOverlay": true
                                                        }
                                                    }
                                                }
                                            },
                                            "trackingParams": "CEcQmY0FGBYiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                        }
                                    }, {
                                        "richItemRenderer": {
                                            "content": {
                                                "videoRenderer": {
                                                    "videoId": "Fky4E8iO4Vk",
                                                    "thumbnail": {
                                                        "thumbnails": [{
                                                            "url": "https://i.ytimg.com/vi/Fky4E8iO4Vk/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLChU7CEUgsqyvhngTp-KsvztG8ANw",
                                                            "width": 168,
                                                            "height": 94
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/Fky4E8iO4Vk/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLBgdNDERKodoSsU6jJYGF3us9bFUw",
                                                            "width": 196,
                                                            "height": 110
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/Fky4E8iO4Vk/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLC46J8bb35gs5L_nhxO6FnD6Db0jg",
                                                            "width": 246,
                                                            "height": 138
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/Fky4E8iO4Vk/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLDoH38ts5k9D-PNsphR4CHeMp-xjQ",
                                                            "width": 336,
                                                            "height": 188
                                                        }]
                                                    },
                                                    "title": {
                                                        "runs": [{
                                                            "text": "9 .Kubernetes集群搭建 |  Vagrant快速创建虚拟机"
                                                        }],
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "9 .Kubernetes集群搭建 |  Vagrant快速创建虚拟机 by 麦兜搞IT 7 months ago 5 minutes, 19 seconds 194 views"
                                                            }
                                                        }
                                                    },
                                                    "descriptionSnippet": {
                                                        "runs": [{
                                                            "text": "欢迎大家购买完整课程《Certified Kubernetes Administrator (CKA) 考试完全指南（2022版）》https://www.udemy.com/course/k8s-chinese/?referralCode=4D8B7AFDBFAF9A8E4F81\n\n#k8s..."
                                                        }]
                                                    },
                                                    "publishedTimeText": {
                                                        "simpleText": "7 months ago"
                                                    },
                                                    "lengthText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "5 minutes, 19 seconds"
                                                            }
                                                        },
                                                        "simpleText": "5:19"
                                                    },
                                                    "viewCountText": {
                                                        "simpleText": "194 views"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CEIQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxaGFVDbWpkaHdNR1N1dDhtWjFDcW5SampVd5oBBhDyOBjgB6oBGlVVTEZtamRod01HU3V0OG1aMUNxblJqalV3",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "url": "/watch?v=Fky4E8iO4Vk",
                                                                "webPageType": "WEB_PAGE_TYPE_WATCH",
                                                                "rootVe": 3832
                                                            }
                                                        },
                                                        "watchEndpoint": {
                                                            "videoId": "Fky4E8iO4Vk",
                                                            "watchEndpointSupportedOnesieConfig": {
                                                                "html5PlaybackOnesieConfig": {
                                                                    "commonConfig": {
                                                                        "url": "https://rr7---sn-cnoa-jv3s.googlevideo.com/initplayback?source=youtube\u0026oeis=1\u0026c=WEB\u0026oad=3200\u0026ovd=3200\u0026oaad=11000\u0026oavd=11000\u0026ocs=700\u0026oewis=1\u0026oputc=1\u0026ofpcc=1\u0026msp=1\u0026odepv=1\u0026id=164cb813c88ee159\u0026ip=117.213.17.165\u0026initcwndbps=977500\u0026mt=1679566621\u0026oweuc=\u0026pxtags=Cg4KAnR4EggyNDQyODQxNQ\u0026rxtags=Cg4KAnR4EggyNDQyODQxNA%2CCg4KAnR4EggyNDQyODQxNQ%2CCg4KAnR4EggyNDQyODQxNg%2CCg4KAnR4EggyNDQyODQxNw%2CCg4KAnR4EggyNDQyODQxOA%2CCg4KAnR4EggyNDQyODQxOQ%2CCg4KAnR4EggyNDQyODQyMA"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CEIQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxA2cK7xLyCrqYWqgEaVVVMRm1qZGh3TUdTdXQ4bVoxQ3FuUmpqVXc=",
                                                    "showActionMenu": false,
                                                    "shortViewCountText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "194 views"
                                                            }
                                                        },
                                                        "simpleText": "194 views"
                                                    },
                                                    "menu": {
                                                        "menuRenderer": {
                                                            "items": [{
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Add to queue"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "ADD_TO_QUEUE_TAIL"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CEYQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true
                                                                            }
                                                                        },
                                                                        "signalServiceEndpoint": {
                                                                            "signal": "CLIENT_SIGNAL",
                                                                            "actions": [{
                                                                                "clickTrackingParams": "CEYQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "addToPlaylistCommand": {
                                                                                    "openMiniplayer": true,
                                                                                    "videoId": "Fky4E8iO4Vk",
                                                                                    "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                                    "onCreateListCommand": {
                                                                                        "clickTrackingParams": "CEYQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                        "commandMetadata": {
                                                                                            "webCommandMetadata": {
                                                                                                "sendPost": true,
                                                                                                "apiUrl": "/youtubei/v1/playlist/create"
                                                                                            }
                                                                                        },
                                                                                        "createPlaylistServiceEndpoint": {
                                                                                            "videoIds": ["Fky4E8iO4Vk"],
                                                                                            "params": "CAQ%3D"
                                                                                        }
                                                                                    },
                                                                                    "videoIds": ["Fky4E8iO4Vk"]
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CEYQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }, {
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Share"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "SHARE"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CEIQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true,
                                                                                "apiUrl": "/youtubei/v1/share/get_share_panel"
                                                                            }
                                                                        },
                                                                        "shareEntityServiceEndpoint": {
                                                                            "serializedShareEntity": "CgtGa3k0RThpTzRWaw%3D%3D",
                                                                            "commands": [{
                                                                                "clickTrackingParams": "CEIQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "openPopupAction": {
                                                                                    "popup": {
                                                                                        "unifiedSharePanelRenderer": {
                                                                                            "trackingParams": "CEUQjmIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                            "showLoadingSpinner": true
                                                                                        }
                                                                                    },
                                                                                    "popupType": "DIALOG",
                                                                                    "beReused": true
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CEIQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }],
                                                            "trackingParams": "CEIQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                            "accessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Action menu"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "thumbnailOverlays": [{
                                                        "thumbnailOverlayTimeStatusRenderer": {
                                                            "text": {
                                                                "accessibility": {
                                                                    "accessibilityData": {
                                                                        "label": "5 minutes, 19 seconds"
                                                                    }
                                                                },
                                                                "simpleText": "5:19"
                                                            },
                                                            "style": "DEFAULT"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "isToggled": false,
                                                            "untoggledIcon": {
                                                                "iconType": "WATCH_LATER"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "CHECK"
                                                            },
                                                            "untoggledTooltip": "Watch later",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CEQQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "addedVideoId": "Fky4E8iO4Vk",
                                                                        "action": "ACTION_ADD_VIDEO"
                                                                    }]
                                                                }
                                                            },
                                                            "toggledServiceEndpoint": {
                                                                "clickTrackingParams": "CEQQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "action": "ACTION_REMOVE_VIDEO_BY_VIDEO_ID",
                                                                        "removedVideoId": "Fky4E8iO4Vk"
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Watch later"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CEQQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "untoggledIcon": {
                                                                "iconType": "ADD_TO_QUEUE_TAIL"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "PLAYLIST_ADD_CHECK"
                                                            },
                                                            "untoggledTooltip": "Add to queue",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CEMQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true
                                                                    }
                                                                },
                                                                "signalServiceEndpoint": {
                                                                    "signal": "CLIENT_SIGNAL",
                                                                    "actions": [{
                                                                        "clickTrackingParams": "CEMQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "addToPlaylistCommand": {
                                                                            "openMiniplayer": true,
                                                                            "videoId": "Fky4E8iO4Vk",
                                                                            "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                            "onCreateListCommand": {
                                                                                "clickTrackingParams": "CEMQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "commandMetadata": {
                                                                                    "webCommandMetadata": {
                                                                                        "sendPost": true,
                                                                                        "apiUrl": "/youtubei/v1/playlist/create"
                                                                                    }
                                                                                },
                                                                                "createPlaylistServiceEndpoint": {
                                                                                    "videoIds": ["Fky4E8iO4Vk"],
                                                                                    "params": "CAQ%3D"
                                                                                }
                                                                            },
                                                                            "videoIds": ["Fky4E8iO4Vk"]
                                                                        }
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Add to queue"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CEMQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayNowPlayingRenderer": {
                                                            "text": {
                                                                "runs": [{
                                                                    "text": "Now playing"
                                                                }]
                                                            }
                                                        }
                                                    }],
                                                    "richThumbnail": {
                                                        "movingThumbnailRenderer": {
                                                            "movingThumbnailDetails": {
                                                                "thumbnails": [{
                                                                    "url": "https://i.ytimg.com/an_webp/Fky4E8iO4Vk/mqdefault_6s.webp?du=3000\u0026sqp=CLfA8KAG\u0026rs=AOn4CLCko8KeCwy-TYQ9b3Gj9VTRF-pTzw",
                                                                    "width": 320,
                                                                    "height": 180
                                                                }],
                                                                "logAsMovingThumbnail": true
                                                            },
                                                            "enableHoveredLogging": true,
                                                            "enableOverlay": true
                                                        }
                                                    }
                                                }
                                            },
                                            "trackingParams": "CEEQmY0FGBciEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                        }
                                    }, {
                                        "richItemRenderer": {
                                            "content": {
                                                "videoRenderer": {
                                                    "videoId": "WM-34rydj6M",
                                                    "thumbnail": {
                                                        "thumbnails": [{
                                                            "url": "https://i.ytimg.com/vi/WM-34rydj6M/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLCTAQbTtQtyamcGxS1bp3SxlLhRlQ",
                                                            "width": 168,
                                                            "height": 94
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/WM-34rydj6M/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLAWiP5jYlyayh9QxGNMK7cpoifDRw",
                                                            "width": 196,
                                                            "height": 110
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/WM-34rydj6M/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLBxlKEkwwd9-Ka8h-uSn68tfc95tQ",
                                                            "width": 246,
                                                            "height": 138
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/WM-34rydj6M/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLApGJdmm_jmWe3VHNLrZ-dVu11f-A",
                                                            "width": 336,
                                                            "height": 188
                                                        }]
                                                    },
                                                    "title": {
                                                        "runs": [{
                                                            "text": "8.Kubernetes集群搭建 |  kubeadm本地搭建集群的验证"
                                                        }],
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "8.Kubernetes集群搭建 |  kubeadm本地搭建集群的验证 by 麦兜搞IT 7 months ago 6 minutes, 54 seconds 68 views"
                                                            }
                                                        }
                                                    },
                                                    "descriptionSnippet": {
                                                        "runs": [{
                                                            "text": "欢迎大家购买完整课程《Certified Kubernetes Administrator (CKA) 考试完全指南（2022版）》https://www.udemy.com/course/k8s-chinese/?referralCode=4D8B7AFDBFAF9A8E4F81\n\n#k8s..."
                                                        }]
                                                    },
                                                    "publishedTimeText": {
                                                        "simpleText": "7 months ago"
                                                    },
                                                    "lengthText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "6 minutes, 54 seconds"
                                                            }
                                                        },
                                                        "simpleText": "6:54"
                                                    },
                                                    "viewCountText": {
                                                        "simpleText": "68 views"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CDwQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxaGFVDbWpkaHdNR1N1dDhtWjFDcW5SampVd5oBBhDyOBjgB6oBGlVVTEZtamRod01HU3V0OG1aMUNxblJqalV3",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "url": "/watch?v=WM-34rydj6M",
                                                                "webPageType": "WEB_PAGE_TYPE_WATCH",
                                                                "rootVe": 3832
                                                            }
                                                        },
                                                        "watchEndpoint": {
                                                            "videoId": "WM-34rydj6M",
                                                            "watchEndpointSupportedOnesieConfig": {
                                                                "html5PlaybackOnesieConfig": {
                                                                    "commonConfig": {
                                                                        "url": "https://rr2---sn-cnoa-jv3l.googlevideo.com/initplayback?source=youtube\u0026oeis=1\u0026c=WEB\u0026oad=3200\u0026ovd=3200\u0026oaad=11000\u0026oavd=11000\u0026ocs=700\u0026oewis=1\u0026oputc=1\u0026ofpcc=1\u0026msp=1\u0026odepv=1\u0026id=58cfb7e2bc9d8fa3\u0026ip=117.213.17.165\u0026initcwndbps=1068750\u0026mt=1679566621\u0026oweuc=\u0026pxtags=Cg4KAnR4EggyNDQyODQxNQ\u0026rxtags=Cg4KAnR4EggyNDQyODQxNA%2CCg4KAnR4EggyNDQyODQxNQ%2CCg4KAnR4EggyNDQyODQxNg%2CCg4KAnR4EggyNDQyODQxNw%2CCg4KAnR4EggyNDQyODQxOA%2CCg4KAnR4EggyNDQyODQxOQ%2CCg4KAnR4EggyNDQyODQyMA"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CDwQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxAo5_25Kv87edYqgEaVVVMRm1qZGh3TUdTdXQ4bVoxQ3FuUmpqVXc=",
                                                    "showActionMenu": false,
                                                    "shortViewCountText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "68 views"
                                                            }
                                                        },
                                                        "simpleText": "68 views"
                                                    },
                                                    "menu": {
                                                        "menuRenderer": {
                                                            "items": [{
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Add to queue"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "ADD_TO_QUEUE_TAIL"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CEAQ_pgEGAUiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true
                                                                            }
                                                                        },
                                                                        "signalServiceEndpoint": {
                                                                            "signal": "CLIENT_SIGNAL",
                                                                            "actions": [{
                                                                                "clickTrackingParams": "CEAQ_pgEGAUiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "addToPlaylistCommand": {
                                                                                    "openMiniplayer": true,
                                                                                    "videoId": "WM-34rydj6M",
                                                                                    "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                                    "onCreateListCommand": {
                                                                                        "clickTrackingParams": "CEAQ_pgEGAUiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                        "commandMetadata": {
                                                                                            "webCommandMetadata": {
                                                                                                "sendPost": true,
                                                                                                "apiUrl": "/youtubei/v1/playlist/create"
                                                                                            }
                                                                                        },
                                                                                        "createPlaylistServiceEndpoint": {
                                                                                            "videoIds": ["WM-34rydj6M"],
                                                                                            "params": "CAQ%3D"
                                                                                        }
                                                                                    },
                                                                                    "videoIds": ["WM-34rydj6M"]
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CEAQ_pgEGAUiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }, {
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Share"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "SHARE"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CDwQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true,
                                                                                "apiUrl": "/youtubei/v1/share/get_share_panel"
                                                                            }
                                                                        },
                                                                        "shareEntityServiceEndpoint": {
                                                                            "serializedShareEntity": "CgtXTS0zNHJ5ZGo2TQ%3D%3D",
                                                                            "commands": [{
                                                                                "clickTrackingParams": "CDwQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "openPopupAction": {
                                                                                    "popup": {
                                                                                        "unifiedSharePanelRenderer": {
                                                                                            "trackingParams": "CD8QjmIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                            "showLoadingSpinner": true
                                                                                        }
                                                                                    },
                                                                                    "popupType": "DIALOG",
                                                                                    "beReused": true
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CDwQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }],
                                                            "trackingParams": "CDwQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                            "accessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Action menu"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "thumbnailOverlays": [{
                                                        "thumbnailOverlayTimeStatusRenderer": {
                                                            "text": {
                                                                "accessibility": {
                                                                    "accessibilityData": {
                                                                        "label": "6 minutes, 54 seconds"
                                                                    }
                                                                },
                                                                "simpleText": "6:54"
                                                            },
                                                            "style": "DEFAULT"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "isToggled": false,
                                                            "untoggledIcon": {
                                                                "iconType": "WATCH_LATER"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "CHECK"
                                                            },
                                                            "untoggledTooltip": "Watch later",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CD4Q-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "addedVideoId": "WM-34rydj6M",
                                                                        "action": "ACTION_ADD_VIDEO"
                                                                    }]
                                                                }
                                                            },
                                                            "toggledServiceEndpoint": {
                                                                "clickTrackingParams": "CD4Q-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "action": "ACTION_REMOVE_VIDEO_BY_VIDEO_ID",
                                                                        "removedVideoId": "WM-34rydj6M"
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Watch later"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CD4Q-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "untoggledIcon": {
                                                                "iconType": "ADD_TO_QUEUE_TAIL"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "PLAYLIST_ADD_CHECK"
                                                            },
                                                            "untoggledTooltip": "Add to queue",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CD0Qx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true
                                                                    }
                                                                },
                                                                "signalServiceEndpoint": {
                                                                    "signal": "CLIENT_SIGNAL",
                                                                    "actions": [{
                                                                        "clickTrackingParams": "CD0Qx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "addToPlaylistCommand": {
                                                                            "openMiniplayer": true,
                                                                            "videoId": "WM-34rydj6M",
                                                                            "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                            "onCreateListCommand": {
                                                                                "clickTrackingParams": "CD0Qx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "commandMetadata": {
                                                                                    "webCommandMetadata": {
                                                                                        "sendPost": true,
                                                                                        "apiUrl": "/youtubei/v1/playlist/create"
                                                                                    }
                                                                                },
                                                                                "createPlaylistServiceEndpoint": {
                                                                                    "videoIds": ["WM-34rydj6M"],
                                                                                    "params": "CAQ%3D"
                                                                                }
                                                                            },
                                                                            "videoIds": ["WM-34rydj6M"]
                                                                        }
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Add to queue"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CD0Qx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayNowPlayingRenderer": {
                                                            "text": {
                                                                "runs": [{
                                                                    "text": "Now playing"
                                                                }]
                                                            }
                                                        }
                                                    }]
                                                }
                                            },
                                            "trackingParams": "CDsQmY0FGBgiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                        }
                                    }, {
                                        "richItemRenderer": {
                                            "content": {
                                                "videoRenderer": {
                                                    "videoId": "L83mbrFkbRo",
                                                    "thumbnail": {
                                                        "thumbnails": [{
                                                            "url": "https://i.ytimg.com/vi/L83mbrFkbRo/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLBKi6HG1jm-IAIhzpc8kriJVYzQvA",
                                                            "width": 168,
                                                            "height": 94
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/L83mbrFkbRo/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLDqDmkx8PKjJRJSzA0LjXd3LbReQQ",
                                                            "width": 196,
                                                            "height": 110
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/L83mbrFkbRo/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLDrdI5U023hPzSURimalrNSUOjqCw",
                                                            "width": 246,
                                                            "height": 138
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/L83mbrFkbRo/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLCYRsfflLvOmNScCnmMkPB3XJ5jMw",
                                                            "width": 336,
                                                            "height": 188
                                                        }]
                                                    },
                                                    "title": {
                                                        "runs": [{
                                                            "text": "7.Kubernetes集群搭建 |  修复Node的Internal IP问题"
                                                        }],
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "7.Kubernetes集群搭建 |  修复Node的Internal IP问题 by 麦兜搞IT 7 months ago 5 minutes, 3 seconds 58 views"
                                                            }
                                                        }
                                                    },
                                                    "descriptionSnippet": {
                                                        "runs": [{
                                                            "text": "欢迎大家购买完整课程《Certified Kubernetes Administrator (CKA) 考试完全指南（2022版）》https://www.udemy.com/course/k8s-chinese/?referralCode=4D8B7AFDBFAF9A8E4F81\n\n#k8s..."
                                                        }]
                                                    },
                                                    "publishedTimeText": {
                                                        "simpleText": "7 months ago"
                                                    },
                                                    "lengthText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "5 minutes, 3 seconds"
                                                            }
                                                        },
                                                        "simpleText": "5:03"
                                                    },
                                                    "viewCountText": {
                                                        "simpleText": "58 views"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CDYQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxaGFVDbWpkaHdNR1N1dDhtWjFDcW5SampVd5oBBhDyOBjgB6oBGlVVTEZtamRod01HU3V0OG1aMUNxblJqalV3",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "url": "/watch?v=L83mbrFkbRo",
                                                                "webPageType": "WEB_PAGE_TYPE_WATCH",
                                                                "rootVe": 3832
                                                            }
                                                        },
                                                        "watchEndpoint": {
                                                            "videoId": "L83mbrFkbRo",
                                                            "watchEndpointSupportedOnesieConfig": {
                                                                "html5PlaybackOnesieConfig": {
                                                                    "commonConfig": {
                                                                        "url": "https://rr1---sn-cnoa-jv3s.googlevideo.com/initplayback?source=youtube\u0026oeis=1\u0026c=WEB\u0026oad=3200\u0026ovd=3200\u0026oaad=11000\u0026oavd=11000\u0026ocs=700\u0026oewis=1\u0026oputc=1\u0026ofpcc=1\u0026msp=1\u0026odepv=1\u0026id=2fcde66eb1646d1a\u0026ip=117.213.17.165\u0026initcwndbps=977500\u0026mt=1679566621\u0026oweuc=\u0026pxtags=Cg4KAnR4EggyNDQyODQxNQ\u0026rxtags=Cg4KAnR4EggyNDQyODQxNA%2CCg4KAnR4EggyNDQyODQxNQ%2CCg4KAnR4EggyNDQyODQxNg%2CCg4KAnR4EggyNDQyODQxNw%2CCg4KAnR4EggyNDQyODQxOA%2CCg4KAnR4EggyNDQyODQxOQ%2CCg4KAnR4EggyNDQyODQyMA"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CDYQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxAmtqRi-vN-eYvqgEaVVVMRm1qZGh3TUdTdXQ4bVoxQ3FuUmpqVXc=",
                                                    "showActionMenu": false,
                                                    "shortViewCountText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "58 views"
                                                            }
                                                        },
                                                        "simpleText": "58 views"
                                                    },
                                                    "menu": {
                                                        "menuRenderer": {
                                                            "items": [{
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Add to queue"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "ADD_TO_QUEUE_TAIL"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CDoQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true
                                                                            }
                                                                        },
                                                                        "signalServiceEndpoint": {
                                                                            "signal": "CLIENT_SIGNAL",
                                                                            "actions": [{
                                                                                "clickTrackingParams": "CDoQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "addToPlaylistCommand": {
                                                                                    "openMiniplayer": true,
                                                                                    "videoId": "L83mbrFkbRo",
                                                                                    "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                                    "onCreateListCommand": {
                                                                                        "clickTrackingParams": "CDoQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                        "commandMetadata": {
                                                                                            "webCommandMetadata": {
                                                                                                "sendPost": true,
                                                                                                "apiUrl": "/youtubei/v1/playlist/create"
                                                                                            }
                                                                                        },
                                                                                        "createPlaylistServiceEndpoint": {
                                                                                            "videoIds": ["L83mbrFkbRo"],
                                                                                            "params": "CAQ%3D"
                                                                                        }
                                                                                    },
                                                                                    "videoIds": ["L83mbrFkbRo"]
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CDoQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }, {
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Share"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "SHARE"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CDYQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true,
                                                                                "apiUrl": "/youtubei/v1/share/get_share_panel"
                                                                            }
                                                                        },
                                                                        "shareEntityServiceEndpoint": {
                                                                            "serializedShareEntity": "CgtMODNtYnJGa2JSbw%3D%3D",
                                                                            "commands": [{
                                                                                "clickTrackingParams": "CDYQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "openPopupAction": {
                                                                                    "popup": {
                                                                                        "unifiedSharePanelRenderer": {
                                                                                            "trackingParams": "CDkQjmIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                            "showLoadingSpinner": true
                                                                                        }
                                                                                    },
                                                                                    "popupType": "DIALOG",
                                                                                    "beReused": true
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CDYQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }],
                                                            "trackingParams": "CDYQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                            "accessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Action menu"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "thumbnailOverlays": [{
                                                        "thumbnailOverlayTimeStatusRenderer": {
                                                            "text": {
                                                                "accessibility": {
                                                                    "accessibilityData": {
                                                                        "label": "5 minutes, 3 seconds"
                                                                    }
                                                                },
                                                                "simpleText": "5:03"
                                                            },
                                                            "style": "DEFAULT"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "isToggled": false,
                                                            "untoggledIcon": {
                                                                "iconType": "WATCH_LATER"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "CHECK"
                                                            },
                                                            "untoggledTooltip": "Watch later",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CDgQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "addedVideoId": "L83mbrFkbRo",
                                                                        "action": "ACTION_ADD_VIDEO"
                                                                    }]
                                                                }
                                                            },
                                                            "toggledServiceEndpoint": {
                                                                "clickTrackingParams": "CDgQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "action": "ACTION_REMOVE_VIDEO_BY_VIDEO_ID",
                                                                        "removedVideoId": "L83mbrFkbRo"
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Watch later"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CDgQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "untoggledIcon": {
                                                                "iconType": "ADD_TO_QUEUE_TAIL"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "PLAYLIST_ADD_CHECK"
                                                            },
                                                            "untoggledTooltip": "Add to queue",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CDcQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true
                                                                    }
                                                                },
                                                                "signalServiceEndpoint": {
                                                                    "signal": "CLIENT_SIGNAL",
                                                                    "actions": [{
                                                                        "clickTrackingParams": "CDcQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "addToPlaylistCommand": {
                                                                            "openMiniplayer": true,
                                                                            "videoId": "L83mbrFkbRo",
                                                                            "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                            "onCreateListCommand": {
                                                                                "clickTrackingParams": "CDcQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "commandMetadata": {
                                                                                    "webCommandMetadata": {
                                                                                        "sendPost": true,
                                                                                        "apiUrl": "/youtubei/v1/playlist/create"
                                                                                    }
                                                                                },
                                                                                "createPlaylistServiceEndpoint": {
                                                                                    "videoIds": ["L83mbrFkbRo"],
                                                                                    "params": "CAQ%3D"
                                                                                }
                                                                            },
                                                                            "videoIds": ["L83mbrFkbRo"]
                                                                        }
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Add to queue"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CDcQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayNowPlayingRenderer": {
                                                            "text": {
                                                                "runs": [{
                                                                    "text": "Now playing"
                                                                }]
                                                            }
                                                        }
                                                    }],
                                                    "richThumbnail": {
                                                        "movingThumbnailRenderer": {
                                                            "movingThumbnailDetails": {
                                                                "thumbnails": [{
                                                                    "url": "https://i.ytimg.com/an_webp/L83mbrFkbRo/mqdefault_6s.webp?du=3000\u0026sqp=CN7C8KAG\u0026rs=AOn4CLADP1qwZA8r7KvaXDjfGXOUxBcuhg",
                                                                    "width": 320,
                                                                    "height": 180
                                                                }],
                                                                "logAsMovingThumbnail": true
                                                            },
                                                            "enableHoveredLogging": true,
                                                            "enableOverlay": true
                                                        }
                                                    }
                                                }
                                            },
                                            "trackingParams": "CDUQmY0FGBkiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                        }
                                    }, {
                                        "richItemRenderer": {
                                            "content": {
                                                "videoRenderer": {
                                                    "videoId": "j5rmtgyP8vY",
                                                    "thumbnail": {
                                                        "thumbnails": [{
                                                            "url": "https://i.ytimg.com/vi/j5rmtgyP8vY/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLBW8AIytRuMDAKE7Bq0ko5DWhsVjQ",
                                                            "width": 168,
                                                            "height": 94
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/j5rmtgyP8vY/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLDYNuIV-CNqLAYl6YVvnxvUe0ah0Q",
                                                            "width": 196,
                                                            "height": 110
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/j5rmtgyP8vY/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLADeCU2bmfuWw2ssuCJR20T0BYztg",
                                                            "width": 246,
                                                            "height": 138
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/j5rmtgyP8vY/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLAGMncPwW3R9lo_ipk84nZZBXjn_w",
                                                            "width": 336,
                                                            "height": 188
                                                        }]
                                                    },
                                                    "title": {
                                                        "runs": [{
                                                            "text": "6.Kubernetes集群搭建 |  kubeadm本地搭建3节点集群"
                                                        }],
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "6.Kubernetes集群搭建 |  kubeadm本地搭建3节点集群 by 麦兜搞IT 7 months ago 21 minutes 374 views"
                                                            }
                                                        }
                                                    },
                                                    "descriptionSnippet": {
                                                        "runs": [{
                                                            "text": "欢迎大家购买完整课程《Certified Kubernetes Administrator (CKA) 考试完全指南（2022版）》https://www.udemy.com/course/k8s-chinese/?referralCode=4D8B7AFDBFAF9A8E4F81\n\n#k8s..."
                                                        }]
                                                    },
                                                    "publishedTimeText": {
                                                        "simpleText": "7 months ago"
                                                    },
                                                    "lengthText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "21 minutes, 41 seconds"
                                                            }
                                                        },
                                                        "simpleText": "21:41"
                                                    },
                                                    "viewCountText": {
                                                        "simpleText": "374 views"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CDAQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxaGFVDbWpkaHdNR1N1dDhtWjFDcW5SampVd5oBBhDyOBjgB6oBGlVVTEZtamRod01HU3V0OG1aMUNxblJqalV3",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "url": "/watch?v=j5rmtgyP8vY",
                                                                "webPageType": "WEB_PAGE_TYPE_WATCH",
                                                                "rootVe": 3832
                                                            }
                                                        },
                                                        "watchEndpoint": {
                                                            "videoId": "j5rmtgyP8vY",
                                                            "watchEndpointSupportedOnesieConfig": {
                                                                "html5PlaybackOnesieConfig": {
                                                                    "commonConfig": {
                                                                        "url": "https://rr2---sn-cnoa-jv3s.googlevideo.com/initplayback?source=youtube\u0026oeis=1\u0026c=WEB\u0026oad=3200\u0026ovd=3200\u0026oaad=11000\u0026oavd=11000\u0026ocs=700\u0026oewis=1\u0026oputc=1\u0026ofpcc=1\u0026msp=1\u0026odepv=1\u0026id=8f9ae6b60c8ff2f6\u0026ip=117.213.17.165\u0026initcwndbps=977500\u0026mt=1679566621\u0026oweuc=\u0026pxtags=Cg4KAnR4EggyNDQyODQxNQ\u0026rxtags=Cg4KAnR4EggyNDQyODQxNA%2CCg4KAnR4EggyNDQyODQxNQ%2CCg4KAnR4EggyNDQyODQxNg%2CCg4KAnR4EggyNDQyODQxNw%2CCg4KAnR4EggyNDQyODQxOA%2CCg4KAnR4EggyNDQyODQxOQ%2CCg4KAnR4EggyNDQyODQyMA"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CDAQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxA9uW_5ODWuc2PAaoBGlVVTEZtamRod01HU3V0OG1aMUNxblJqalV3",
                                                    "showActionMenu": false,
                                                    "shortViewCountText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "374 views"
                                                            }
                                                        },
                                                        "simpleText": "374 views"
                                                    },
                                                    "menu": {
                                                        "menuRenderer": {
                                                            "items": [{
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Add to queue"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "ADD_TO_QUEUE_TAIL"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CDQQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true
                                                                            }
                                                                        },
                                                                        "signalServiceEndpoint": {
                                                                            "signal": "CLIENT_SIGNAL",
                                                                            "actions": [{
                                                                                "clickTrackingParams": "CDQQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "addToPlaylistCommand": {
                                                                                    "openMiniplayer": true,
                                                                                    "videoId": "j5rmtgyP8vY",
                                                                                    "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                                    "onCreateListCommand": {
                                                                                        "clickTrackingParams": "CDQQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                        "commandMetadata": {
                                                                                            "webCommandMetadata": {
                                                                                                "sendPost": true,
                                                                                                "apiUrl": "/youtubei/v1/playlist/create"
                                                                                            }
                                                                                        },
                                                                                        "createPlaylistServiceEndpoint": {
                                                                                            "videoIds": ["j5rmtgyP8vY"],
                                                                                            "params": "CAQ%3D"
                                                                                        }
                                                                                    },
                                                                                    "videoIds": ["j5rmtgyP8vY"]
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CDQQ_pgEGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }, {
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Share"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "SHARE"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CDAQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true,
                                                                                "apiUrl": "/youtubei/v1/share/get_share_panel"
                                                                            }
                                                                        },
                                                                        "shareEntityServiceEndpoint": {
                                                                            "serializedShareEntity": "CgtqNXJtdGd5UDh2WQ%3D%3D",
                                                                            "commands": [{
                                                                                "clickTrackingParams": "CDAQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "openPopupAction": {
                                                                                    "popup": {
                                                                                        "unifiedSharePanelRenderer": {
                                                                                            "trackingParams": "CDMQjmIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                            "showLoadingSpinner": true
                                                                                        }
                                                                                    },
                                                                                    "popupType": "DIALOG",
                                                                                    "beReused": true
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CDAQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }],
                                                            "trackingParams": "CDAQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                            "accessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Action menu"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "thumbnailOverlays": [{
                                                        "thumbnailOverlayTimeStatusRenderer": {
                                                            "text": {
                                                                "accessibility": {
                                                                    "accessibilityData": {
                                                                        "label": "21 minutes, 41 seconds"
                                                                    }
                                                                },
                                                                "simpleText": "21:41"
                                                            },
                                                            "style": "DEFAULT"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "isToggled": false,
                                                            "untoggledIcon": {
                                                                "iconType": "WATCH_LATER"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "CHECK"
                                                            },
                                                            "untoggledTooltip": "Watch later",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CDIQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "addedVideoId": "j5rmtgyP8vY",
                                                                        "action": "ACTION_ADD_VIDEO"
                                                                    }]
                                                                }
                                                            },
                                                            "toggledServiceEndpoint": {
                                                                "clickTrackingParams": "CDIQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "action": "ACTION_REMOVE_VIDEO_BY_VIDEO_ID",
                                                                        "removedVideoId": "j5rmtgyP8vY"
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Watch later"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CDIQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "untoggledIcon": {
                                                                "iconType": "ADD_TO_QUEUE_TAIL"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "PLAYLIST_ADD_CHECK"
                                                            },
                                                            "untoggledTooltip": "Add to queue",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CDEQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true
                                                                    }
                                                                },
                                                                "signalServiceEndpoint": {
                                                                    "signal": "CLIENT_SIGNAL",
                                                                    "actions": [{
                                                                        "clickTrackingParams": "CDEQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "addToPlaylistCommand": {
                                                                            "openMiniplayer": true,
                                                                            "videoId": "j5rmtgyP8vY",
                                                                            "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                            "onCreateListCommand": {
                                                                                "clickTrackingParams": "CDEQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "commandMetadata": {
                                                                                    "webCommandMetadata": {
                                                                                        "sendPost": true,
                                                                                        "apiUrl": "/youtubei/v1/playlist/create"
                                                                                    }
                                                                                },
                                                                                "createPlaylistServiceEndpoint": {
                                                                                    "videoIds": ["j5rmtgyP8vY"],
                                                                                    "params": "CAQ%3D"
                                                                                }
                                                                            },
                                                                            "videoIds": ["j5rmtgyP8vY"]
                                                                        }
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Add to queue"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CDEQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayNowPlayingRenderer": {
                                                            "text": {
                                                                "runs": [{
                                                                    "text": "Now playing"
                                                                }]
                                                            }
                                                        }
                                                    }],
                                                    "richThumbnail": {
                                                        "movingThumbnailRenderer": {
                                                            "movingThumbnailDetails": {
                                                                "thumbnails": [{
                                                                    "url": "https://i.ytimg.com/an_webp/j5rmtgyP8vY/mqdefault_6s.webp?du=3000\u0026sqp=CMyg8KAG\u0026rs=AOn4CLC79oMLWFqzbxGVEG5pa0qKgkx75Q",
                                                                    "width": 320,
                                                                    "height": 180
                                                                }],
                                                                "logAsMovingThumbnail": true
                                                            },
                                                            "enableHoveredLogging": true,
                                                            "enableOverlay": true
                                                        }
                                                    }
                                                }
                                            },
                                            "trackingParams": "CC8QmY0FGBoiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                        }
                                    }, {
                                        "richItemRenderer": {
                                            "content": {
                                                "videoRenderer": {
                                                    "videoId": "kFlcGtaZ7Kc",
                                                    "thumbnail": {
                                                        "thumbnails": [{
                                                            "url": "https://i.ytimg.com/vi/kFlcGtaZ7Kc/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLClh8wYOOmtGBknykSphZ7mqEJMHw",
                                                            "width": 168,
                                                            "height": 94
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/kFlcGtaZ7Kc/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLCoNTpffj0RXijB6KiBIaXlp-vYXg",
                                                            "width": 196,
                                                            "height": 110
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/kFlcGtaZ7Kc/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLCtMyZJKDnq4ClbK1VIjctcrSTTrA",
                                                            "width": 246,
                                                            "height": 138
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/kFlcGtaZ7Kc/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLBZjCU7bOXOCMHux_LNQUlMhbWokA",
                                                            "width": 336,
                                                            "height": 188
                                                        }]
                                                    },
                                                    "title": {
                                                        "runs": [{
                                                            "text": "5.Kubernetes集群搭建 | minikube快速搭建本地集群Mac"
                                                        }],
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "5.Kubernetes集群搭建 | minikube快速搭建本地集群Mac by 麦兜搞IT 7 months ago 8 minutes, 8 seconds 103 views"
                                                            }
                                                        }
                                                    },
                                                    "descriptionSnippet": {
                                                        "runs": [{
                                                            "text": "欢迎大家购买完整课程《Certified Kubernetes Administrator (CKA) 考试完全指南（2022版）》https://www.udemy.com/course/k8s-chinese/?referralCode=4D8B7AFDBFAF9A8E4F81\n\n#k8s..."
                                                        }]
                                                    },
                                                    "publishedTimeText": {
                                                        "simpleText": "7 months ago"
                                                    },
                                                    "lengthText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "8 minutes, 8 seconds"
                                                            }
                                                        },
                                                        "simpleText": "8:08"
                                                    },
                                                    "viewCountText": {
                                                        "simpleText": "103 views"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CCoQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxaGFVDbWpkaHdNR1N1dDhtWjFDcW5SampVd5oBBhDyOBjgB6oBGlVVTEZtamRod01HU3V0OG1aMUNxblJqalV3",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "url": "/watch?v=kFlcGtaZ7Kc",
                                                                "webPageType": "WEB_PAGE_TYPE_WATCH",
                                                                "rootVe": 3832
                                                            }
                                                        },
                                                        "watchEndpoint": {
                                                            "videoId": "kFlcGtaZ7Kc",
                                                            "watchEndpointSupportedOnesieConfig": {
                                                                "html5PlaybackOnesieConfig": {
                                                                    "commonConfig": {
                                                                        "url": "https://rr4---sn-cnoa-jv3s.googlevideo.com/initplayback?source=youtube\u0026oeis=1\u0026c=WEB\u0026oad=3200\u0026ovd=3200\u0026oaad=11000\u0026oavd=11000\u0026ocs=700\u0026oewis=1\u0026oputc=1\u0026ofpcc=1\u0026msp=1\u0026odepv=1\u0026id=90595c1ad699eca7\u0026ip=117.213.17.165\u0026initcwndbps=977500\u0026mt=1679566621\u0026oweuc=\u0026pxtags=Cg4KAnR4EggyNDQyODQxNQ\u0026rxtags=Cg4KAnR4EggyNDQyODQxNA%2CCg4KAnR4EggyNDQyODQxNQ%2CCg4KAnR4EggyNDQyODQxNg%2CCg4KAnR4EggyNDQyODQxNw%2CCg4KAnR4EggyNDQyODQxOA%2CCg4KAnR4EggyNDQyODQxOQ%2CCg4KAnR4EggyNDQyODQyMA"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CCoQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxAp9nntK2D16yQAaoBGlVVTEZtamRod01HU3V0OG1aMUNxblJqalV3",
                                                    "showActionMenu": false,
                                                    "shortViewCountText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "103 views"
                                                            }
                                                        },
                                                        "simpleText": "103 views"
                                                    },
                                                    "menu": {
                                                        "menuRenderer": {
                                                            "items": [{
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Add to queue"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "ADD_TO_QUEUE_TAIL"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CC4Q_pgEGAUiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true
                                                                            }
                                                                        },
                                                                        "signalServiceEndpoint": {
                                                                            "signal": "CLIENT_SIGNAL",
                                                                            "actions": [{
                                                                                "clickTrackingParams": "CC4Q_pgEGAUiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "addToPlaylistCommand": {
                                                                                    "openMiniplayer": true,
                                                                                    "videoId": "kFlcGtaZ7Kc",
                                                                                    "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                                    "onCreateListCommand": {
                                                                                        "clickTrackingParams": "CC4Q_pgEGAUiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                        "commandMetadata": {
                                                                                            "webCommandMetadata": {
                                                                                                "sendPost": true,
                                                                                                "apiUrl": "/youtubei/v1/playlist/create"
                                                                                            }
                                                                                        },
                                                                                        "createPlaylistServiceEndpoint": {
                                                                                            "videoIds": ["kFlcGtaZ7Kc"],
                                                                                            "params": "CAQ%3D"
                                                                                        }
                                                                                    },
                                                                                    "videoIds": ["kFlcGtaZ7Kc"]
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CC4Q_pgEGAUiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }, {
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Share"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "SHARE"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CCoQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true,
                                                                                "apiUrl": "/youtubei/v1/share/get_share_panel"
                                                                            }
                                                                        },
                                                                        "shareEntityServiceEndpoint": {
                                                                            "serializedShareEntity": "CgtrRmxjR3RhWjdLYw%3D%3D",
                                                                            "commands": [{
                                                                                "clickTrackingParams": "CCoQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "openPopupAction": {
                                                                                    "popup": {
                                                                                        "unifiedSharePanelRenderer": {
                                                                                            "trackingParams": "CC0QjmIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                            "showLoadingSpinner": true
                                                                                        }
                                                                                    },
                                                                                    "popupType": "DIALOG",
                                                                                    "beReused": true
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CCoQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }],
                                                            "trackingParams": "CCoQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                            "accessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Action menu"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "thumbnailOverlays": [{
                                                        "thumbnailOverlayTimeStatusRenderer": {
                                                            "text": {
                                                                "accessibility": {
                                                                    "accessibilityData": {
                                                                        "label": "8 minutes, 8 seconds"
                                                                    }
                                                                },
                                                                "simpleText": "8:08"
                                                            },
                                                            "style": "DEFAULT"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "isToggled": false,
                                                            "untoggledIcon": {
                                                                "iconType": "WATCH_LATER"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "CHECK"
                                                            },
                                                            "untoggledTooltip": "Watch later",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CCwQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "addedVideoId": "kFlcGtaZ7Kc",
                                                                        "action": "ACTION_ADD_VIDEO"
                                                                    }]
                                                                }
                                                            },
                                                            "toggledServiceEndpoint": {
                                                                "clickTrackingParams": "CCwQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "action": "ACTION_REMOVE_VIDEO_BY_VIDEO_ID",
                                                                        "removedVideoId": "kFlcGtaZ7Kc"
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Watch later"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CCwQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "untoggledIcon": {
                                                                "iconType": "ADD_TO_QUEUE_TAIL"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "PLAYLIST_ADD_CHECK"
                                                            },
                                                            "untoggledTooltip": "Add to queue",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CCsQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true
                                                                    }
                                                                },
                                                                "signalServiceEndpoint": {
                                                                    "signal": "CLIENT_SIGNAL",
                                                                    "actions": [{
                                                                        "clickTrackingParams": "CCsQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "addToPlaylistCommand": {
                                                                            "openMiniplayer": true,
                                                                            "videoId": "kFlcGtaZ7Kc",
                                                                            "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                            "onCreateListCommand": {
                                                                                "clickTrackingParams": "CCsQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "commandMetadata": {
                                                                                    "webCommandMetadata": {
                                                                                        "sendPost": true,
                                                                                        "apiUrl": "/youtubei/v1/playlist/create"
                                                                                    }
                                                                                },
                                                                                "createPlaylistServiceEndpoint": {
                                                                                    "videoIds": ["kFlcGtaZ7Kc"],
                                                                                    "params": "CAQ%3D"
                                                                                }
                                                                            },
                                                                            "videoIds": ["kFlcGtaZ7Kc"]
                                                                        }
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Add to queue"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CCsQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayNowPlayingRenderer": {
                                                            "text": {
                                                                "runs": [{
                                                                    "text": "Now playing"
                                                                }]
                                                            }
                                                        }
                                                    }]
                                                }
                                            },
                                            "trackingParams": "CCkQmY0FGBsiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                        }
                                    }, {
                                        "richItemRenderer": {
                                            "content": {
                                                "videoRenderer": {
                                                    "videoId": "SmkA35QvW6I",
                                                    "thumbnail": {
                                                        "thumbnails": [{
                                                            "url": "https://i.ytimg.com/vi/SmkA35QvW6I/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLAn9IdFaOHYQL7TtDmuW8BIOh6gnw",
                                                            "width": 168,
                                                            "height": 94
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/SmkA35QvW6I/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLD8B35bDWdZSlfzzmfDml-2-rvN4A",
                                                            "width": 196,
                                                            "height": 110
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/SmkA35QvW6I/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLBJZTNw-GiMBaMXxOKXthsAuNu-pg",
                                                            "width": 246,
                                                            "height": 138
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/SmkA35QvW6I/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLDCwEDUi3MdGdxaJ8nzAhGbRqivSA",
                                                            "width": 336,
                                                            "height": 188
                                                        }]
                                                    },
                                                    "title": {
                                                        "runs": [{
                                                            "text": "4.Kubernetes集群搭建 |  minikube快速搭建本地集群windows"
                                                        }],
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "4.Kubernetes集群搭建 |  minikube快速搭建本地集群windows by 麦兜搞IT 7 months ago 13 minutes, 2 seconds 283 views"
                                                            }
                                                        }
                                                    },
                                                    "descriptionSnippet": {
                                                        "runs": [{
                                                            "text": "欢迎大家购买完整课程《Certified Kubernetes Administrator (CKA) 考试完全指南（2022版）》https://www.udemy.com/course/k8s-chinese/?referralCode=4D8B7AFDBFAF9A8E4F81\n\n#k8s..."
                                                        }]
                                                    },
                                                    "publishedTimeText": {
                                                        "simpleText": "7 months ago"
                                                    },
                                                    "lengthText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "13 minutes, 2 seconds"
                                                            }
                                                        },
                                                        "simpleText": "13:02"
                                                    },
                                                    "viewCountText": {
                                                        "simpleText": "283 views"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CCQQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxaGFVDbWpkaHdNR1N1dDhtWjFDcW5SampVd5oBBhDyOBjgB6oBGlVVTEZtamRod01HU3V0OG1aMUNxblJqalV3",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "url": "/watch?v=SmkA35QvW6I",
                                                                "webPageType": "WEB_PAGE_TYPE_WATCH",
                                                                "rootVe": 3832
                                                            }
                                                        },
                                                        "watchEndpoint": {
                                                            "videoId": "SmkA35QvW6I",
                                                            "watchEndpointSupportedOnesieConfig": {
                                                                "html5PlaybackOnesieConfig": {
                                                                    "commonConfig": {
                                                                        "url": "https://rr2---sn-cnoa-jv3l.googlevideo.com/initplayback?source=youtube\u0026oeis=1\u0026c=WEB\u0026oad=3200\u0026ovd=3200\u0026oaad=11000\u0026oavd=11000\u0026ocs=700\u0026oewis=1\u0026oputc=1\u0026ofpcc=1\u0026msp=1\u0026odepv=1\u0026id=4a6900df942f5ba2\u0026ip=117.213.17.165\u0026initcwndbps=1068750\u0026mt=1679566621\u0026oweuc=\u0026pxtags=Cg4KAnR4EggyNDQyODQxNQ\u0026rxtags=Cg4KAnR4EggyNDQyODQxNA%2CCg4KAnR4EggyNDQyODQxNQ%2CCg4KAnR4EggyNDQyODQxNg%2CCg4KAnR4EggyNDQyODQxNw%2CCg4KAnR4EggyNDQyODQxOA%2CCg4KAnR4EggyNDQyODQxOQ%2CCg4KAnR4EggyNDQyODQyMA"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CCQQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxAore9ofmbwLRKqgEaVVVMRm1qZGh3TUdTdXQ4bVoxQ3FuUmpqVXc=",
                                                    "showActionMenu": false,
                                                    "shortViewCountText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "283 views"
                                                            }
                                                        },
                                                        "simpleText": "283 views"
                                                    },
                                                    "menu": {
                                                        "menuRenderer": {
                                                            "items": [{
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Add to queue"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "ADD_TO_QUEUE_TAIL"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CCgQ_pgEGAUiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true
                                                                            }
                                                                        },
                                                                        "signalServiceEndpoint": {
                                                                            "signal": "CLIENT_SIGNAL",
                                                                            "actions": [{
                                                                                "clickTrackingParams": "CCgQ_pgEGAUiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "addToPlaylistCommand": {
                                                                                    "openMiniplayer": true,
                                                                                    "videoId": "SmkA35QvW6I",
                                                                                    "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                                    "onCreateListCommand": {
                                                                                        "clickTrackingParams": "CCgQ_pgEGAUiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                        "commandMetadata": {
                                                                                            "webCommandMetadata": {
                                                                                                "sendPost": true,
                                                                                                "apiUrl": "/youtubei/v1/playlist/create"
                                                                                            }
                                                                                        },
                                                                                        "createPlaylistServiceEndpoint": {
                                                                                            "videoIds": ["SmkA35QvW6I"],
                                                                                            "params": "CAQ%3D"
                                                                                        }
                                                                                    },
                                                                                    "videoIds": ["SmkA35QvW6I"]
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CCgQ_pgEGAUiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }, {
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Share"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "SHARE"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CCQQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true,
                                                                                "apiUrl": "/youtubei/v1/share/get_share_panel"
                                                                            }
                                                                        },
                                                                        "shareEntityServiceEndpoint": {
                                                                            "serializedShareEntity": "CgtTbWtBMzVRdlc2SQ%3D%3D",
                                                                            "commands": [{
                                                                                "clickTrackingParams": "CCQQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "openPopupAction": {
                                                                                    "popup": {
                                                                                        "unifiedSharePanelRenderer": {
                                                                                            "trackingParams": "CCcQjmIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                            "showLoadingSpinner": true
                                                                                        }
                                                                                    },
                                                                                    "popupType": "DIALOG",
                                                                                    "beReused": true
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CCQQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }],
                                                            "trackingParams": "CCQQ3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                            "accessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Action menu"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "thumbnailOverlays": [{
                                                        "thumbnailOverlayTimeStatusRenderer": {
                                                            "text": {
                                                                "accessibility": {
                                                                    "accessibilityData": {
                                                                        "label": "13 minutes, 2 seconds"
                                                                    }
                                                                },
                                                                "simpleText": "13:02"
                                                            },
                                                            "style": "DEFAULT"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "isToggled": false,
                                                            "untoggledIcon": {
                                                                "iconType": "WATCH_LATER"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "CHECK"
                                                            },
                                                            "untoggledTooltip": "Watch later",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CCYQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "addedVideoId": "SmkA35QvW6I",
                                                                        "action": "ACTION_ADD_VIDEO"
                                                                    }]
                                                                }
                                                            },
                                                            "toggledServiceEndpoint": {
                                                                "clickTrackingParams": "CCYQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "action": "ACTION_REMOVE_VIDEO_BY_VIDEO_ID",
                                                                        "removedVideoId": "SmkA35QvW6I"
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Watch later"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CCYQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "untoggledIcon": {
                                                                "iconType": "ADD_TO_QUEUE_TAIL"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "PLAYLIST_ADD_CHECK"
                                                            },
                                                            "untoggledTooltip": "Add to queue",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CCUQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true
                                                                    }
                                                                },
                                                                "signalServiceEndpoint": {
                                                                    "signal": "CLIENT_SIGNAL",
                                                                    "actions": [{
                                                                        "clickTrackingParams": "CCUQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "addToPlaylistCommand": {
                                                                            "openMiniplayer": true,
                                                                            "videoId": "SmkA35QvW6I",
                                                                            "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                            "onCreateListCommand": {
                                                                                "clickTrackingParams": "CCUQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "commandMetadata": {
                                                                                    "webCommandMetadata": {
                                                                                        "sendPost": true,
                                                                                        "apiUrl": "/youtubei/v1/playlist/create"
                                                                                    }
                                                                                },
                                                                                "createPlaylistServiceEndpoint": {
                                                                                    "videoIds": ["SmkA35QvW6I"],
                                                                                    "params": "CAQ%3D"
                                                                                }
                                                                            },
                                                                            "videoIds": ["SmkA35QvW6I"]
                                                                        }
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Add to queue"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CCUQx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayNowPlayingRenderer": {
                                                            "text": {
                                                                "runs": [{
                                                                    "text": "Now playing"
                                                                }]
                                                            }
                                                        }
                                                    }]
                                                }
                                            },
                                            "trackingParams": "CCMQmY0FGBwiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                        }
                                    }, {
                                        "richItemRenderer": {
                                            "content": {
                                                "videoRenderer": {
                                                    "videoId": "ZAZwEKBW1yU",
                                                    "thumbnail": {
                                                        "thumbnails": [{
                                                            "url": "https://i.ytimg.com/vi/ZAZwEKBW1yU/hqdefault.jpg?sqp=-oaymwEbCKgBEF5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLBaUcal_qOcIukZ0otSj78N2TNr7Q",
                                                            "width": 168,
                                                            "height": 94
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/ZAZwEKBW1yU/hqdefault.jpg?sqp=-oaymwEbCMQBEG5IVPKriqkDDggBFQAAiEIYAXABwAEG\u0026rs=AOn4CLDmn7uhO_VRWNT2X7NZ7-aceAWtBA",
                                                            "width": 196,
                                                            "height": 110
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/ZAZwEKBW1yU/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLCBuVMVxQYA9HoBmS7VhNWogpAE_g",
                                                            "width": 246,
                                                            "height": 138
                                                        }, {
                                                            "url": "https://i.ytimg.com/vi/ZAZwEKBW1yU/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFTyq4qpAw4IARUAAIhCGAFwAcABBg==\u0026rs=AOn4CLAXU2aidmVADoFh8_8qxJPwig8z0g",
                                                            "width": 336,
                                                            "height": 188
                                                        }]
                                                    },
                                                    "title": {
                                                        "runs": [{
                                                            "text": "3.Kubernetes集群搭建 | minikube介绍"
                                                        }],
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "3.Kubernetes集群搭建 | minikube介绍 by 麦兜搞IT 7 months ago 4 minutes, 11 seconds 218 views"
                                                            }
                                                        }
                                                    },
                                                    "descriptionSnippet": {
                                                        "runs": [{
                                                            "text": "欢迎大家购买完整课程《Certified Kubernetes Administrator (CKA) 考试完全指南（2022版）》https://www.udemy.com/course/k8s-chinese/?referralCode=4D8B7AFDBFAF9A8E4F81\n\n#k8s..."
                                                        }]
                                                    },
                                                    "publishedTimeText": {
                                                        "simpleText": "7 months ago"
                                                    },
                                                    "lengthText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "4 minutes, 11 seconds"
                                                            }
                                                        },
                                                        "simpleText": "4:11"
                                                    },
                                                    "viewCountText": {
                                                        "simpleText": "218 views"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CB4Q3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxaGFVDbWpkaHdNR1N1dDhtWjFDcW5SampVd5oBBhDyOBjgB6oBGlVVTEZtamRod01HU3V0OG1aMUNxblJqalV3",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "url": "/watch?v=ZAZwEKBW1yU",
                                                                "webPageType": "WEB_PAGE_TYPE_WATCH",
                                                                "rootVe": 3832
                                                            }
                                                        },
                                                        "watchEndpoint": {
                                                            "videoId": "ZAZwEKBW1yU",
                                                            "watchEndpointSupportedOnesieConfig": {
                                                                "html5PlaybackOnesieConfig": {
                                                                    "commonConfig": {
                                                                        "url": "https://rr4---sn-cnoa-jv3s.googlevideo.com/initplayback?source=youtube\u0026oeis=1\u0026c=WEB\u0026oad=3200\u0026ovd=3200\u0026oaad=11000\u0026oavd=11000\u0026ocs=700\u0026oewis=1\u0026oputc=1\u0026ofpcc=1\u0026msp=1\u0026odepv=1\u0026id=64067010a056d725\u0026ip=117.213.17.165\u0026initcwndbps=977500\u0026mt=1679566621\u0026oweuc=\u0026pxtags=Cg4KAnR4EggyNDQyODQxNQ\u0026rxtags=Cg4KAnR4EggyNDQyODQxNA%2CCg4KAnR4EggyNDQyODQxNQ%2CCg4KAnR4EggyNDQyODQxNg%2CCg4KAnR4EggyNDQyODQxNw%2CCg4KAnR4EggyNDQyODQxOA%2CCg4KAnR4EggyNDQyODQxOQ%2CCg4KAnR4EggyNDQyODQyMA"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CB4Q3DAiEwjH2o6v6vH9AhVk50wCHY1yDbxApa7bgoqCnINkqgEaVVVMRm1qZGh3TUdTdXQ4bVoxQ3FuUmpqVXc=",
                                                    "showActionMenu": false,
                                                    "shortViewCountText": {
                                                        "accessibility": {
                                                            "accessibilityData": {
                                                                "label": "218 views"
                                                            }
                                                        },
                                                        "simpleText": "218 views"
                                                    },
                                                    "menu": {
                                                        "menuRenderer": {
                                                            "items": [{
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Add to queue"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "ADD_TO_QUEUE_TAIL"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CCIQ_pgEGAUiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true
                                                                            }
                                                                        },
                                                                        "signalServiceEndpoint": {
                                                                            "signal": "CLIENT_SIGNAL",
                                                                            "actions": [{
                                                                                "clickTrackingParams": "CCIQ_pgEGAUiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "addToPlaylistCommand": {
                                                                                    "openMiniplayer": true,
                                                                                    "videoId": "ZAZwEKBW1yU",
                                                                                    "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                                    "onCreateListCommand": {
                                                                                        "clickTrackingParams": "CCIQ_pgEGAUiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                        "commandMetadata": {
                                                                                            "webCommandMetadata": {
                                                                                                "sendPost": true,
                                                                                                "apiUrl": "/youtubei/v1/playlist/create"
                                                                                            }
                                                                                        },
                                                                                        "createPlaylistServiceEndpoint": {
                                                                                            "videoIds": ["ZAZwEKBW1yU"],
                                                                                            "params": "CAQ%3D"
                                                                                        }
                                                                                    },
                                                                                    "videoIds": ["ZAZwEKBW1yU"]
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CCIQ_pgEGAUiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }, {
                                                                "menuServiceItemRenderer": {
                                                                    "text": {
                                                                        "runs": [{
                                                                            "text": "Share"
                                                                        }]
                                                                    },
                                                                    "icon": {
                                                                        "iconType": "SHARE"
                                                                    },
                                                                    "serviceEndpoint": {
                                                                        "clickTrackingParams": "CB4Q3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "commandMetadata": {
                                                                            "webCommandMetadata": {
                                                                                "sendPost": true,
                                                                                "apiUrl": "/youtubei/v1/share/get_share_panel"
                                                                            }
                                                                        },
                                                                        "shareEntityServiceEndpoint": {
                                                                            "serializedShareEntity": "CgtaQVp3RUtCVzF5VQ%3D%3D",
                                                                            "commands": [{
                                                                                "clickTrackingParams": "CB4Q3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "openPopupAction": {
                                                                                    "popup": {
                                                                                        "unifiedSharePanelRenderer": {
                                                                                            "trackingParams": "CCEQjmIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                            "showLoadingSpinner": true
                                                                                        }
                                                                                    },
                                                                                    "popupType": "DIALOG",
                                                                                    "beReused": true
                                                                                }
                                                                            }]
                                                                        }
                                                                    },
                                                                    "trackingParams": "CB4Q3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                                }
                                                            }],
                                                            "trackingParams": "CB4Q3DAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                            "accessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Action menu"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "thumbnailOverlays": [{
                                                        "thumbnailOverlayTimeStatusRenderer": {
                                                            "text": {
                                                                "accessibility": {
                                                                    "accessibilityData": {
                                                                        "label": "4 minutes, 11 seconds"
                                                                    }
                                                                },
                                                                "simpleText": "4:11"
                                                            },
                                                            "style": "DEFAULT"
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "isToggled": false,
                                                            "untoggledIcon": {
                                                                "iconType": "WATCH_LATER"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "CHECK"
                                                            },
                                                            "untoggledTooltip": "Watch later",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CCAQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "addedVideoId": "ZAZwEKBW1yU",
                                                                        "action": "ACTION_ADD_VIDEO"
                                                                    }]
                                                                }
                                                            },
                                                            "toggledServiceEndpoint": {
                                                                "clickTrackingParams": "CCAQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true,
                                                                        "apiUrl": "/youtubei/v1/browse/edit_playlist"
                                                                    }
                                                                },
                                                                "playlistEditEndpoint": {
                                                                    "playlistId": "WL",
                                                                    "actions": [{
                                                                        "action": "ACTION_REMOVE_VIDEO_BY_VIDEO_ID",
                                                                        "removedVideoId": "ZAZwEKBW1yU"
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Watch later"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CCAQ-ecDGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayToggleButtonRenderer": {
                                                            "untoggledIcon": {
                                                                "iconType": "ADD_TO_QUEUE_TAIL"
                                                            },
                                                            "toggledIcon": {
                                                                "iconType": "PLAYLIST_ADD_CHECK"
                                                            },
                                                            "untoggledTooltip": "Add to queue",
                                                            "toggledTooltip": "Added",
                                                            "untoggledServiceEndpoint": {
                                                                "clickTrackingParams": "CB8Qx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "sendPost": true
                                                                    }
                                                                },
                                                                "signalServiceEndpoint": {
                                                                    "signal": "CLIENT_SIGNAL",
                                                                    "actions": [{
                                                                        "clickTrackingParams": "CB8Qx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                        "addToPlaylistCommand": {
                                                                            "openMiniplayer": true,
                                                                            "videoId": "ZAZwEKBW1yU",
                                                                            "listType": "PLAYLIST_EDIT_LIST_TYPE_QUEUE",
                                                                            "onCreateListCommand": {
                                                                                "clickTrackingParams": "CB8Qx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                                                                "commandMetadata": {
                                                                                    "webCommandMetadata": {
                                                                                        "sendPost": true,
                                                                                        "apiUrl": "/youtubei/v1/playlist/create"
                                                                                    }
                                                                                },
                                                                                "createPlaylistServiceEndpoint": {
                                                                                    "videoIds": ["ZAZwEKBW1yU"],
                                                                                    "params": "CAQ%3D"
                                                                                }
                                                                            },
                                                                            "videoIds": ["ZAZwEKBW1yU"]
                                                                        }
                                                                    }]
                                                                }
                                                            },
                                                            "untoggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Add to queue"
                                                                }
                                                            },
                                                            "toggledAccessibility": {
                                                                "accessibilityData": {
                                                                    "label": "Added"
                                                                }
                                                            },
                                                            "trackingParams": "CB8Qx-wEGAIiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                                        }
                                                    }, {
                                                        "thumbnailOverlayNowPlayingRenderer": {
                                                            "text": {
                                                                "runs": [{
                                                                    "text": "Now playing"
                                                                }]
                                                            }
                                                        }
                                                    }]
                                                }
                                            },
                                            "trackingParams": "CB0QmY0FGB0iEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                        }
                                    }, {
                                        "continuationItemRenderer": {
                                            "trigger": "CONTINUATION_TRIGGER_ON_ITEM_SHOWN",
                                            "continuationEndpoint": {
                                                "clickTrackingParams": "CBkQ8eIEIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                "commandMetadata": {
                                                    "webCommandMetadata": {
                                                        "sendPost": true,
                                                        "apiUrl": "/youtubei/v1/browse"
                                                    }
                                                },
                                                "continuationCommand": {
                                                    "token": "4qmFsgKjARIYVUNtamRod01HU3V0OG1aMUNxblJqalV3GoYBOGdaZkdsMTZXd3BYQ2k5RloyOUpjR0UzWW1kdmNVTnVTVTVyUzBSSmQwRlVaMlZSWjNOSmVrNUVkMjlCV1ZFMWRYRkVVbXRuUWxWQlFSSWtOalF4WldZM01Ua3RNREF3TUMweU1qaGtMVGxqWW1VdE5UZ3lOREk1WTJOaU1HVTRHQUUlM0Q%3D",
                                                    "request": "CONTINUATION_REQUEST_TYPE_BROWSE"
                                                }
                                            }
                                        }
                                    }],
                                    "trackingParams": "CBkQ8eIEIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                    "header": {
                                        "feedFilterChipBarRenderer": {
                                            "contents": [{
                                                "chipCloudChipRenderer": {
                                                    "text": {
                                                        "simpleText": "Latest"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CBwQ_V0YACITCMfajq_q8f0CFWTnTAIdjXINvA==",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "sendPost": true,
                                                                "apiUrl": "/youtubei/v1/browse"
                                                            }
                                                        },
                                                        "continuationCommand": {
                                                            "token": "4qmFsgJkEhhVQ21qZGh3TUdTdXQ4bVoxQ3FuUmpqVXcaSDhnWXVHaXg2S2hJbUNpUTJOREZsWmpjeE9TMHdNREF3TFRJeU9HUXRPV05pWlMwMU9ESTBNamxqWTJJd1pUZ1lBUSUzRCUzRA%3D%3D",
                                                            "request": "CONTINUATION_REQUEST_TYPE_BROWSE",
                                                            "command": {
                                                                "clickTrackingParams": "CBwQ_V0YACITCMfajq_q8f0CFWTnTAIdjXINvA==",
                                                                "showReloadUiCommand": {
                                                                    "targetId": "641ef719-0000-228d-9cbe-582429ccb0e8"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CBwQ_V0YACITCMfajq_q8f0CFWTnTAIdjXINvA==",
                                                    "isSelected": true
                                                }
                                            }, {
                                                "chipCloudChipRenderer": {
                                                    "text": {
                                                        "simpleText": "Popular"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CBsQ_V0YASITCMfajq_q8f0CFWTnTAIdjXINvA==",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "sendPost": true,
                                                                "apiUrl": "/youtubei/v1/browse"
                                                            }
                                                        },
                                                        "continuationCommand": {
                                                            "token": "4qmFsgJkEhhVQ21qZGh3TUdTdXQ4bVoxQ3FuUmpqVXcaSDhnWXVHaXg2S2hJbUNpUTJOREZsWmpjeE9TMHdNREF3TFRJeU9HUXRPV05pWlMwMU9ESTBNamxqWTJJd1pUZ1lBZyUzRCUzRA%3D%3D",
                                                            "request": "CONTINUATION_REQUEST_TYPE_BROWSE",
                                                            "command": {
                                                                "clickTrackingParams": "CBsQ_V0YASITCMfajq_q8f0CFWTnTAIdjXINvA==",
                                                                "showReloadUiCommand": {
                                                                    "targetId": "641ef719-0000-228d-9cbe-582429ccb0e8"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CBsQ_V0YASITCMfajq_q8f0CFWTnTAIdjXINvA==",
                                                    "isSelected": false
                                                }
                                            }],
                                            "trackingParams": "CBoQ4M4DIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                            "styleType": "FEED_FILTER_CHIP_BAR_STYLE_TYPE_CHANNEL_PAGE_GRID"
                                        }
                                    },
                                    "targetId": "641ef719-0000-228d-9cbe-582429ccb0e8",
                                    "style": "RICH_GRID_STYLE_SLIM"
                                }
                            },
                            "trackingParams": "CBgQ8JMBGAYiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                        }
                    }, {
                        "tabRenderer": {
                            "endpoint": {
                                "clickTrackingParams": "CBcQ8JMBGAciEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                "commandMetadata": {
                                    "webCommandMetadata": {
                                        "url": "/@xiaopeng163/streams",
                                        "webPageType": "WEB_PAGE_TYPE_CHANNEL",
                                        "rootVe": 3611,
                                        "apiUrl": "/youtubei/v1/browse"
                                    }
                                },
                                "browseEndpoint": {
                                    "browseId": "UCmjdhwMGSut8mZ1CqnRjjUw",
                                    "params": "EgdzdHJlYW1z8gYECgJ6AA%3D%3D",
                                    "canonicalBaseUrl": "/@xiaopeng163"
                                }
                            },
                            "title": "Live",
                            "trackingParams": "CBcQ8JMBGAciEwjH2o6v6vH9AhVk50wCHY1yDbw="
                        }
                    }, {
                        "tabRenderer": {
                            "endpoint": {
                                "clickTrackingParams": "CBYQ8JMBGAgiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                "commandMetadata": {
                                    "webCommandMetadata": {
                                        "url": "/@xiaopeng163/playlists",
                                        "webPageType": "WEB_PAGE_TYPE_CHANNEL",
                                        "rootVe": 3611,
                                        "apiUrl": "/youtubei/v1/browse"
                                    }
                                },
                                "browseEndpoint": {
                                    "browseId": "UCmjdhwMGSut8mZ1CqnRjjUw",
                                    "params": "EglwbGF5bGlzdHPyBgQKAkIA",
                                    "canonicalBaseUrl": "/@xiaopeng163"
                                }
                            },
                            "title": "Playlists",
                            "trackingParams": "CBYQ8JMBGAgiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                        }
                    }, {
                        "tabRenderer": {
                            "endpoint": {
                                "clickTrackingParams": "CBUQ8JMBGAkiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                "commandMetadata": {
                                    "webCommandMetadata": {
                                        "url": "/@xiaopeng163/community",
                                        "webPageType": "WEB_PAGE_TYPE_CHANNEL",
                                        "rootVe": 3611,
                                        "apiUrl": "/youtubei/v1/browse"
                                    }
                                },
                                "browseEndpoint": {
                                    "browseId": "UCmjdhwMGSut8mZ1CqnRjjUw",
                                    "params": "Egljb21tdW5pdHnyBgQKAkoA",
                                    "canonicalBaseUrl": "/@xiaopeng163"
                                }
                            },
                            "title": "Community",
                            "trackingParams": "CBUQ8JMBGAkiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                        }
                    }, {
                        "tabRenderer": {
                            "endpoint": {
                                "clickTrackingParams": "CBQQ8JMBGAoiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                "commandMetadata": {
                                    "webCommandMetadata": {
                                        "url": "/@xiaopeng163/channels",
                                        "webPageType": "WEB_PAGE_TYPE_CHANNEL",
                                        "rootVe": 3611,
                                        "apiUrl": "/youtubei/v1/browse"
                                    }
                                },
                                "browseEndpoint": {
                                    "browseId": "UCmjdhwMGSut8mZ1CqnRjjUw",
                                    "params": "EghjaGFubmVsc_IGBAoCUgA%3D",
                                    "canonicalBaseUrl": "/@xiaopeng163"
                                }
                            },
                            "title": "Channels",
                            "trackingParams": "CBQQ8JMBGAoiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                        }
                    }, {
                        "tabRenderer": {
                            "endpoint": {
                                "clickTrackingParams": "CBMQ8JMBGAsiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                "commandMetadata": {
                                    "webCommandMetadata": {
                                        "url": "/@xiaopeng163/about",
                                        "webPageType": "WEB_PAGE_TYPE_CHANNEL",
                                        "rootVe": 3611,
                                        "apiUrl": "/youtubei/v1/browse"
                                    }
                                },
                                "browseEndpoint": {
                                    "browseId": "UCmjdhwMGSut8mZ1CqnRjjUw",
                                    "params": "EgVhYm91dPIGBAoCEgA%3D",
                                    "canonicalBaseUrl": "/@xiaopeng163"
                                }
                            },
                            "title": "About",
                            "trackingParams": "CBMQ8JMBGAsiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                        }
                    }, {
                        "expandableTabRenderer": {
                            "endpoint": {
                                "clickTrackingParams": "CAAQhGciEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                "commandMetadata": {
                                    "webCommandMetadata": {
                                        "url": "/@xiaopeng163/search",
                                        "webPageType": "WEB_PAGE_TYPE_CHANNEL",
                                        "rootVe": 3611,
                                        "apiUrl": "/youtubei/v1/browse"
                                    }
                                },
                                "browseEndpoint": {
                                    "browseId": "UCmjdhwMGSut8mZ1CqnRjjUw",
                                    "params": "EgZzZWFyY2jyBgQKAloA",
                                    "canonicalBaseUrl": "/@xiaopeng163"
                                }
                            },
                            "title": "Search",
                            "selected": false
                        }
                    }]
                }
            },
            "header": {
                "c4TabbedHeaderRenderer": {
                    "channelId": "UCmjdhwMGSut8mZ1CqnRjjUw",
                    "title": "麦兜搞IT",
                    "navigationEndpoint": {
                        "clickTrackingParams": "CBAQ8DsiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                        "commandMetadata": {
                            "webCommandMetadata": {
                                "url": "/@xiaopeng163",
                                "webPageType": "WEB_PAGE_TYPE_CHANNEL",
                                "rootVe": 3611,
                                "apiUrl": "/youtubei/v1/browse"
                            }
                        },
                        "browseEndpoint": {
                            "browseId": "UCmjdhwMGSut8mZ1CqnRjjUw",
                            "canonicalBaseUrl": "/@xiaopeng163"
                        }
                    },
                    "avatar": {
                        "thumbnails": [{
                            "url": "https://yt3.googleusercontent.com/ytc/AL5GRJVEHo8nD-_MA7lwd7BvXCsV09Huj--2dCo59a0=s48-c-k-c0x00ffffff-no-rj",
                            "width": 48,
                            "height": 48
                        }, {
                            "url": "https://yt3.googleusercontent.com/ytc/AL5GRJVEHo8nD-_MA7lwd7BvXCsV09Huj--2dCo59a0=s88-c-k-c0x00ffffff-no-rj",
                            "width": 88,
                            "height": 88
                        }, {
                            "url": "https://yt3.googleusercontent.com/ytc/AL5GRJVEHo8nD-_MA7lwd7BvXCsV09Huj--2dCo59a0=s176-c-k-c0x00ffffff-no-rj",
                            "width": 176,
                            "height": 176
                        }]
                    },
                    "banner": {
                        "thumbnails": [{
                            "url": "https://yt3.googleusercontent.com/MMlw0EuYKgJQSPjBvgucYK5nNKpfAZWO_dYQT1tboqwul1n4ta-jYizIYW75AypN9yo_1qQS=w1060-fcrop64=1,00005a57ffffa5a8-k-c0xffffffff-no-nd-rj",
                            "width": 1060,
                            "height": 175
                        }, {
                            "url": "https://yt3.googleusercontent.com/MMlw0EuYKgJQSPjBvgucYK5nNKpfAZWO_dYQT1tboqwul1n4ta-jYizIYW75AypN9yo_1qQS=w1138-fcrop64=1,00005a57ffffa5a8-k-c0xffffffff-no-nd-rj",
                            "width": 1138,
                            "height": 188
                        }, {
                            "url": "https://yt3.googleusercontent.com/MMlw0EuYKgJQSPjBvgucYK5nNKpfAZWO_dYQT1tboqwul1n4ta-jYizIYW75AypN9yo_1qQS=w1707-fcrop64=1,00005a57ffffa5a8-k-c0xffffffff-no-nd-rj",
                            "width": 1707,
                            "height": 283
                        }, {
                            "url": "https://yt3.googleusercontent.com/MMlw0EuYKgJQSPjBvgucYK5nNKpfAZWO_dYQT1tboqwul1n4ta-jYizIYW75AypN9yo_1qQS=w2120-fcrop64=1,00005a57ffffa5a8-k-c0xffffffff-no-nd-rj",
                            "width": 2120,
                            "height": 351
                        }, {
                            "url": "https://yt3.googleusercontent.com/MMlw0EuYKgJQSPjBvgucYK5nNKpfAZWO_dYQT1tboqwul1n4ta-jYizIYW75AypN9yo_1qQS=w2276-fcrop64=1,00005a57ffffa5a8-k-c0xffffffff-no-nd-rj",
                            "width": 2276,
                            "height": 377
                        }, {
                            "url": "https://yt3.googleusercontent.com/MMlw0EuYKgJQSPjBvgucYK5nNKpfAZWO_dYQT1tboqwul1n4ta-jYizIYW75AypN9yo_1qQS=w2560-fcrop64=1,00005a57ffffa5a8-k-c0xffffffff-no-nd-rj",
                            "width": 2560,
                            "height": 424
                        }]
                    },
                    "subscribeButton": {
                        "buttonRenderer": {
                            "style": "STYLE_DESTRUCTIVE",
                            "size": "SIZE_DEFAULT",
                            "isDisabled": false,
                            "text": {
                                "runs": [{
                                    "text": "Subscribe"
                                }]
                            },
                            "navigationEndpoint": {
                                "clickTrackingParams": "CBEQ8FsiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                "commandMetadata": {
                                    "webCommandMetadata": {
                                        "ignoreNavigation": true
                                    }
                                },
                                "modalEndpoint": {
                                    "modal": {
                                        "modalWithTitleAndButtonRenderer": {
                                            "title": {
                                                "simpleText": "Want to subscribe to this channel?"
                                            },
                                            "content": {
                                                "simpleText": "Sign in to subscribe to this channel."
                                            },
                                            "button": {
                                                "buttonRenderer": {
                                                    "style": "STYLE_BLUE_TEXT",
                                                    "size": "SIZE_DEFAULT",
                                                    "isDisabled": false,
                                                    "text": {
                                                        "simpleText": "Sign in"
                                                    },
                                                    "navigationEndpoint": {
                                                        "clickTrackingParams": "CBIQ_YYEIhMIx9qOr-rx_QIVZOdMAh2Ncg28MglzdWJzY3JpYmU=",
                                                        "commandMetadata": {
                                                            "webCommandMetadata": {
                                                                "url": "https://accounts.google.com/ServiceLogin?service=youtube\u0026uilel=3\u0026passive=true\u0026continue=https%3A%2F%2Fwww.youtube.com%2Fsignin%3Faction_handle_signin%3Dtrue%26app%3Ddesktop%26hl%3Den%26next%3D%252F%2540xiaopeng163%252Fvideos%26continue_action%3DQUFFLUhqa0NxQUZYM1lfWUlqQ1pMTEIyMWlxbE9hNjVlQXxBQ3Jtc0ttZ0hQb19oekhwSko2TlFQUjFMalllNi1TZU1xbGgyVF9qTUF1b1NMa2JDbVBZLWUzUzF0OGxhaUNhNXVxTjRad0ZYWXR4MDJ4LTBGcGFQZmNMREgzVnNmc0stSVBqUmlhbXVhUzd1RUFBYldxLUdMSkNYQVNSRDJuclgyOW1RMFRtZjUzNkY1aUhwLVJjVmtDakJIV2d3dHMybGdaMmpYRFJHckxvVFhyRGZsNllLMHNOQWRxMHhyRHVwSnBXdXlKc1hKSTY\u0026hl=en\u0026ec=66429",
                                                                "webPageType": "WEB_PAGE_TYPE_UNKNOWN",
                                                                "rootVe": 83769
                                                            }
                                                        },
                                                        "signInEndpoint": {
                                                            "nextEndpoint": {
                                                                "clickTrackingParams": "CBIQ_YYEIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                                "commandMetadata": {
                                                                    "webCommandMetadata": {
                                                                        "url": "/@xiaopeng163/videos",
                                                                        "webPageType": "WEB_PAGE_TYPE_CHANNEL",
                                                                        "rootVe": 3611,
                                                                        "apiUrl": "/youtubei/v1/browse"
                                                                    }
                                                                },
                                                                "browseEndpoint": {
                                                                    "browseId": "UCmjdhwMGSut8mZ1CqnRjjUw",
                                                                    "params": "EgZ2aWRlb3M%3D",
                                                                    "canonicalBaseUrl": "/@xiaopeng163"
                                                                }
                                                            },
                                                            "continueAction": "QUFFLUhqa0NxQUZYM1lfWUlqQ1pMTEIyMWlxbE9hNjVlQXxBQ3Jtc0ttZ0hQb19oekhwSko2TlFQUjFMalllNi1TZU1xbGgyVF9qTUF1b1NMa2JDbVBZLWUzUzF0OGxhaUNhNXVxTjRad0ZYWXR4MDJ4LTBGcGFQZmNMREgzVnNmc0stSVBqUmlhbXVhUzd1RUFBYldxLUdMSkNYQVNSRDJuclgyOW1RMFRtZjUzNkY1aUhwLVJjVmtDakJIV2d3dHMybGdaMmpYRFJHckxvVFhyRGZsNllLMHNOQWRxMHhyRHVwSnBXdXlKc1hKSTY",
                                                            "idamTag": "66429"
                                                        }
                                                    },
                                                    "trackingParams": "CBIQ_YYEIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            "trackingParams": "CBEQ8FsiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                        }
                    },
                    "visitTracking": {
                        "remarketingPing": "https://www.youtube.com/pagead/viewthroughconversion/962985656/?backend=innertube\u0026cname=1\u0026cver=2_20230320_07_00\u0026data=backend%3Dinnertube%3Bcname%3D1%3Bcver%3D2_20230320_07_00%3Bptype%3Dcview%3Btype%3Dcview%3Butuid%3DmjdhwMGSut8mZ1CqnRjjUw\u0026foc_id=mjdhwMGSut8mZ1CqnRjjUw\u0026label=followon_cvisit\u0026ptype=cview\u0026utuid=mjdhwMGSut8mZ1CqnRjjUw"
                    },
                    "subscriberCountText": {
                        "accessibility": {
                            "accessibilityData": {
                                "label": "7.18K subscribers"
                            }
                        },
                        "simpleText": "7.18K subscribers"
                    },
                    "tvBanner": {
                        "thumbnails": [{
                            "url": "https://yt3.googleusercontent.com/MMlw0EuYKgJQSPjBvgucYK5nNKpfAZWO_dYQT1tboqwul1n4ta-jYizIYW75AypN9yo_1qQS=w320-fcrop64=1,00000000ffffffff-k-c0xffffffff-no-nd-rj",
                            "width": 320,
                            "height": 180
                        }, {
                            "url": "https://yt3.googleusercontent.com/MMlw0EuYKgJQSPjBvgucYK5nNKpfAZWO_dYQT1tboqwul1n4ta-jYizIYW75AypN9yo_1qQS=w854-fcrop64=1,00000000ffffffff-k-c0xffffffff-no-nd-rj",
                            "width": 854,
                            "height": 480
                        }, {
                            "url": "https://yt3.googleusercontent.com/MMlw0EuYKgJQSPjBvgucYK5nNKpfAZWO_dYQT1tboqwul1n4ta-jYizIYW75AypN9yo_1qQS=w1280-fcrop64=1,00000000ffffffff-k-c0xffffffff-no-nd-rj",
                            "width": 1280,
                            "height": 720
                        }, {
                            "url": "https://yt3.googleusercontent.com/MMlw0EuYKgJQSPjBvgucYK5nNKpfAZWO_dYQT1tboqwul1n4ta-jYizIYW75AypN9yo_1qQS=w1920-fcrop64=1,00000000ffffffff-k-c0xffffffff-no-nd-rj",
                            "width": 1920,
                            "height": 1080
                        }, {
                            "url": "https://yt3.googleusercontent.com/MMlw0EuYKgJQSPjBvgucYK5nNKpfAZWO_dYQT1tboqwul1n4ta-jYizIYW75AypN9yo_1qQS=w2120-fcrop64=1,00000000ffffffff-k-c0xffffffff-no-nd-rj",
                            "width": 2120,
                            "height": 1192
                        }]
                    },
                    "mobileBanner": {
                        "thumbnails": [{
                            "url": "https://yt3.googleusercontent.com/MMlw0EuYKgJQSPjBvgucYK5nNKpfAZWO_dYQT1tboqwul1n4ta-jYizIYW75AypN9yo_1qQS=w320-fcrop64=1,32b75a57cd48a5a8-k-c0xffffffff-no-nd-rj",
                            "width": 320,
                            "height": 88
                        }, {
                            "url": "https://yt3.googleusercontent.com/MMlw0EuYKgJQSPjBvgucYK5nNKpfAZWO_dYQT1tboqwul1n4ta-jYizIYW75AypN9yo_1qQS=w640-fcrop64=1,32b75a57cd48a5a8-k-c0xffffffff-no-nd-rj",
                            "width": 640,
                            "height": 175
                        }, {
                            "url": "https://yt3.googleusercontent.com/MMlw0EuYKgJQSPjBvgucYK5nNKpfAZWO_dYQT1tboqwul1n4ta-jYizIYW75AypN9yo_1qQS=w960-fcrop64=1,32b75a57cd48a5a8-k-c0xffffffff-no-nd-rj",
                            "width": 960,
                            "height": 263
                        }, {
                            "url": "https://yt3.googleusercontent.com/MMlw0EuYKgJQSPjBvgucYK5nNKpfAZWO_dYQT1tboqwul1n4ta-jYizIYW75AypN9yo_1qQS=w1280-fcrop64=1,32b75a57cd48a5a8-k-c0xffffffff-no-nd-rj",
                            "width": 1280,
                            "height": 351
                        }, {
                            "url": "https://yt3.googleusercontent.com/MMlw0EuYKgJQSPjBvgucYK5nNKpfAZWO_dYQT1tboqwul1n4ta-jYizIYW75AypN9yo_1qQS=w1440-fcrop64=1,32b75a57cd48a5a8-k-c0xffffffff-no-nd-rj",
                            "width": 1440,
                            "height": 395
                        }]
                    },
                    "trackingParams": "CBAQ8DsiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                    "channelHandleText": {
                        "runs": [{
                            "text": "@xiaopeng163"
                        }]
                    },
                    "videosCountText": {
                        "runs": [{
                            "text": "311"
                        }, {
                            "text": " videos"
                        }]
                    }
                }
            },
            "metadata": {
                "channelMetadataRenderer": {
                    "title": "麦兜搞IT",
                    "description": "Welcome to my channel,  this Channel focus on technology like networking, automation, Python, Docker, kubernetes, git, etc.\n\n大家好，欢迎大家来到我的频道，我目前在荷兰海牙工作。\n\nB站账户麦兜搞IT https://space.bilibili.com/364122352   微信公众号 ”卖逗搞IT“",
                    "rssUrl": "https://www.youtube.com/feeds/videos.xml?channel_id=UCmjdhwMGSut8mZ1CqnRjjUw",
                    "channelConversionUrl": "https://www.youtube.com/pagead/viewthroughconversion/962985656/?backend=innertube\u0026cname=1\u0026cver=2_20230320_07_00\u0026data=backend%3Dinnertube%3Bcname%3D1%3Bcver%3D2_20230320_07_00%3Bptype%3Dcview%3Btype%3Dcview%3Butuid%3DmjdhwMGSut8mZ1CqnRjjUw\u0026foc_id=mjdhwMGSut8mZ1CqnRjjUw\u0026label=followon_cvisit\u0026ptype=cview\u0026utuid=mjdhwMGSut8mZ1CqnRjjUw",
                    "externalId": "UCmjdhwMGSut8mZ1CqnRjjUw",
                    "keywords": "Technology",
                    "ownerUrls": ["http://www.youtube.com/@xiaopeng163"],
                    "avatar": {
                        "thumbnails": [{
                            "url": "https://yt3.googleusercontent.com/ytc/AL5GRJVEHo8nD-_MA7lwd7BvXCsV09Huj--2dCo59a0=s900-c-k-c0x00ffffff-no-rj",
                            "width": 900,
                            "height": 900
                        }]
                    },
                    "channelUrl": "https://www.youtube.com/channel/UCmjdhwMGSut8mZ1CqnRjjUw",
                    "isFamilySafe": true,
                    "availableCountryCodes": ["CR", "SV", "SS", "VC", "GA", "ID", "LB", "MT", "IE", "NF", "GG", "MX", "AD", "HR", "KW", "NE", "TF", "FO", "BG", "FM", "EH", "AU", "LS", "SJ", "UY", "SD", "UA", "AW", "EC", "TO", "TH", "GI", "AI", "LC", "MH", "VA", "IO", "PN", "NZ", "AE", "SB", "IS", "FI", "ES", "BL", "CW", "GF", "TK", "MO", "EE", "TN", "PM", "TW", "MW", "GB", "PL", "UM", "JP", "IQ", "MR", "RO", "SX", "AS", "HT", "JM", "ET", "ER", "NC", "FJ", "BI", "PW", "LU", "VN", "TD", "CA", "CN", "RW", "OM", "TL", "VU", "SE", "SG", "KR", "GH", "PG", "TJ", "PE", "SA", "MQ", "PR", "PS", "SK", "IR", "BQ", "CV", "LV", "MA", "AR", "RE", "VI", "CZ", "MK", "GS", "MV", "PF", "NL", "RU", "GT", "AL", "BH", "BR", "BT", "TR", "CF", "UG", "UZ", "ML", "CU", "HM", "PK", "SI", "SO", "TG", "DM", "LK", "AM", "AT", "JO", "BB", "LR", "SR", "ZM", "CL", "PT", "FR", "BW", "AO", "ZA", "BE", "GM", "AF", "GN", "NR", "DJ", "IN", "KE", "IT", "PH", "PY", "BY", "LA", "DO", "YT", "LY", "JE", "CX", "KH", "GR", "BN", "MZ", "BS", "CO", "AZ", "NA", "SY", "BD", "VG", "SZ", "HU", "HK", "KY", "WF", "KG", "MP", "TZ", "NG", "KZ", "ZW", "GD", "BV", "CI", "RS", "CY", "QA", "LI", "YE", "IL", "SM", "CK", "NP", "SH", "AX", "AG", "MC", "SL", "CM", "CG", "MG", "GQ", "BM", "DE", "NI", "AQ", "ME", "NO", "PA", "GU", "MY", "CD", "BO", "MM", "SC", "KM", "TT", "VE", "TM", "NU", "MN", "MF", "HN", "LT", "BZ", "FK", "BA", "MD", "ST", "BJ", "EG", "WS", "TV", "MS", "US", "DZ", "GE", "BF", "SN", "GY", "IM", "GP", "KI", "CH", "DK", "KP", "GL", "GW", "CC", "MU", "TC", "KN"],
                    "androidDeepLink": "android-app://com.google.android.youtube/http/www.youtube.com/channel/UCmjdhwMGSut8mZ1CqnRjjUw",
                    "androidAppindexingLink": "android-app://com.google.android.youtube/http/www.youtube.com/channel/UCmjdhwMGSut8mZ1CqnRjjUw",
                    "iosAppindexingLink": "ios-app://544007664/vnd.youtube/www.youtube.com/channel/UCmjdhwMGSut8mZ1CqnRjjUw",
                    "vanityChannelUrl": "http://www.youtube.com/@xiaopeng163"
                }
            },
            "trackingParams": "CAAQhGciEwjH2o6v6vH9AhVk50wCHY1yDbw=",
            "topbar": {
                "desktopTopbarRenderer": {
                    "logo": {
                        "topbarLogoRenderer": {
                            "iconImage": {
                                "iconType": "YOUTUBE_LOGO"
                            },
                            "tooltipText": {
                                "runs": [{
                                    "text": "YouTube Home"
                                }]
                            },
                            "endpoint": {
                                "clickTrackingParams": "CA8QsV4iEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                "commandMetadata": {
                                    "webCommandMetadata": {
                                        "url": "/",
                                        "webPageType": "WEB_PAGE_TYPE_BROWSE",
                                        "rootVe": 3854,
                                        "apiUrl": "/youtubei/v1/browse"
                                    }
                                },
                                "browseEndpoint": {
                                    "browseId": "FEwhat_to_watch"
                                }
                            },
                            "trackingParams": "CA8QsV4iEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                            "overrideEntityKey": "EgZ0b3BiYXIg9QEoAQ%3D%3D"
                        }
                    },
                    "searchbox": {
                        "fusionSearchboxRenderer": {
                            "icon": {
                                "iconType": "SEARCH"
                            },
                            "placeholderText": {
                                "runs": [{
                                    "text": "Search"
                                }]
                            },
                            "config": {
                                "webSearchboxConfig": {
                                    "requestLanguage": "en",
                                    "requestDomain": "in",
                                    "hasOnscreenKeyboard": false,
                                    "focusSearchbox": true
                                }
                            },
                            "trackingParams": "CA0Q7VAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                            "searchEndpoint": {
                                "clickTrackingParams": "CA0Q7VAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                "commandMetadata": {
                                    "webCommandMetadata": {
                                        "url": "/results?search_query=",
                                        "webPageType": "WEB_PAGE_TYPE_SEARCH",
                                        "rootVe": 4724
                                    }
                                },
                                "searchEndpoint": {
                                    "query": ""
                                }
                            },
                            "clearButton": {
                                "buttonRenderer": {
                                    "style": "STYLE_DEFAULT",
                                    "size": "SIZE_DEFAULT",
                                    "isDisabled": false,
                                    "icon": {
                                        "iconType": "CLOSE"
                                    },
                                    "trackingParams": "CA4Q8FsiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                    "accessibilityData": {
                                        "accessibilityData": {
                                            "label": "Clear search query"
                                        }
                                    }
                                }
                            }
                        }
                    },
                    "trackingParams": "CAEQq6wBIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                    "countryCode": "IN",
                    "topbarButtons": [{
                        "topbarMenuButtonRenderer": {
                            "icon": {
                                "iconType": "MORE_VERT"
                            },
                            "menuRequest": {
                                "clickTrackingParams": "CAsQ_qsBGAAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                "commandMetadata": {
                                    "webCommandMetadata": {
                                        "sendPost": true,
                                        "apiUrl": "/youtubei/v1/account/account_menu"
                                    }
                                },
                                "signalServiceEndpoint": {
                                    "signal": "GET_ACCOUNT_MENU",
                                    "actions": [{
                                        "clickTrackingParams": "CAsQ_qsBGAAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                        "openPopupAction": {
                                            "popup": {
                                                "multiPageMenuRenderer": {
                                                    "trackingParams": "CAwQ_6sBIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                    "style": "MULTI_PAGE_MENU_STYLE_TYPE_SYSTEM",
                                                    "showLoadingSpinner": true
                                                }
                                            },
                                            "popupType": "DROPDOWN",
                                            "beReused": true
                                        }
                                    }]
                                }
                            },
                            "trackingParams": "CAsQ_qsBGAAiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                            "accessibility": {
                                "accessibilityData": {
                                    "label": "Settings"
                                }
                            },
                            "tooltip": "Settings",
                            "style": "STYLE_DEFAULT"
                        }
                    }, {
                        "buttonRenderer": {
                            "style": "STYLE_SUGGESTIVE",
                            "size": "SIZE_SMALL",
                            "text": {
                                "runs": [{
                                    "text": "Sign in"
                                }]
                            },
                            "icon": {
                                "iconType": "AVATAR_LOGGED_OUT"
                            },
                            "navigationEndpoint": {
                                "clickTrackingParams": "CAoQ1IAEGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                "commandMetadata": {
                                    "webCommandMetadata": {
                                        "url": "https://accounts.google.com/ServiceLogin?service=youtube\u0026uilel=3\u0026passive=true\u0026continue=https%3A%2F%2Fwww.youtube.com%2Fsignin%3Faction_handle_signin%3Dtrue%26app%3Ddesktop%26hl%3Den%26next%3Dhttps%253A%252F%252Fwww.youtube.com%252F%2540xiaopeng163%252Fvideos\u0026hl=en\u0026ec=65620",
                                        "webPageType": "WEB_PAGE_TYPE_UNKNOWN",
                                        "rootVe": 83769
                                    }
                                },
                                "signInEndpoint": {
                                    "idamTag": "65620"
                                }
                            },
                            "trackingParams": "CAoQ1IAEGAEiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                            "targetId": "topbar-signin"
                        }
                    }],
                    "hotkeyDialog": {
                        "hotkeyDialogRenderer": {
                            "title": {
                                "runs": [{
                                    "text": "Keyboard shortcuts"
                                }]
                            },
                            "sections": [{
                                "hotkeyDialogSectionRenderer": {
                                    "title": {
                                        "runs": [{
                                            "text": "Playback"
                                        }]
                                    },
                                    "options": [{
                                        "hotkeyDialogSectionOptionRenderer": {
                                            "label": {
                                                "runs": [{
                                                    "text": "Toggle play/pause"
                                                }]
                                            },
                                            "hotkey": "k"
                                        }
                                    }, {
                                        "hotkeyDialogSectionOptionRenderer": {
                                            "label": {
                                                "runs": [{
                                                    "text": "Rewind 10 seconds"
                                                }]
                                            },
                                            "hotkey": "j"
                                        }
                                    }, {
                                        "hotkeyDialogSectionOptionRenderer": {
                                            "label": {
                                                "runs": [{
                                                    "text": "Fast forward 10 seconds"
                                                }]
                                            },
                                            "hotkey": "l"
                                        }
                                    }, {
                                        "hotkeyDialogSectionOptionRenderer": {
                                            "label": {
                                                "runs": [{
                                                    "text": "Previous video"
                                                }]
                                            },
                                            "hotkey": "P (SHIFT+p)"
                                        }
                                    }, {
                                        "hotkeyDialogSectionOptionRenderer": {
                                            "label": {
                                                "runs": [{
                                                    "text": "Next video"
                                                }]
                                            },
                                            "hotkey": "N (SHIFT+n)"
                                        }
                                    }, {
                                        "hotkeyDialogSectionOptionRenderer": {
                                            "label": {
                                                "runs": [{
                                                    "text": "Previous frame (while paused)"
                                                }]
                                            },
                                            "hotkey": ",",
                                            "hotkeyAccessibilityLabel": {
                                                "accessibilityData": {
                                                    "label": "Comma"
                                                }
                                            }
                                        }
                                    }, {
                                        "hotkeyDialogSectionOptionRenderer": {
                                            "label": {
                                                "runs": [{
                                                    "text": "Next frame (while paused)"
                                                }]
                                            },
                                            "hotkey": ".",
                                            "hotkeyAccessibilityLabel": {
                                                "accessibilityData": {
                                                    "label": "Period"
                                                }
                                            }
                                        }
                                    }, {
                                        "hotkeyDialogSectionOptionRenderer": {
                                            "label": {
                                                "runs": [{
                                                    "text": "Decrease playback rate"
                                                }]
                                            },
                                            "hotkey": "\u003c (SHIFT+,)",
                                            "hotkeyAccessibilityLabel": {
                                                "accessibilityData": {
                                                    "label": "Less than or SHIFT + comma"
                                                }
                                            }
                                        }
                                    }, {
                                        "hotkeyDialogSectionOptionRenderer": {
                                            "label": {
                                                "runs": [{
                                                    "text": "Increase playback rate"
                                                }]
                                            },
                                            "hotkey": "\u003e (SHIFT+.)",
                                            "hotkeyAccessibilityLabel": {
                                                "accessibilityData": {
                                                    "label": "Greater than or SHIFT + period"
                                                }
                                            }
                                        }
                                    }, {
                                        "hotkeyDialogSectionOptionRenderer": {
                                            "label": {
                                                "runs": [{
                                                    "text": "Seek to specific point in the video (7 advances to 70% of duration)"
                                                }]
                                            },
                                            "hotkey": "0..9"
                                        }
                                    }, {
                                        "hotkeyDialogSectionOptionRenderer": {
                                            "label": {
                                                "runs": [{
                                                    "text": "Seek to previous chapter"
                                                }]
                                            },
                                            "hotkey": "CONTROL + ←"
                                        }
                                    }, {
                                        "hotkeyDialogSectionOptionRenderer": {
                                            "label": {
                                                "runs": [{
                                                    "text": "Seek to next chapter"
                                                }]
                                            },
                                            "hotkey": "CONTROL + →"
                                        }
                                    }]
                                }
                            }, {
                                "hotkeyDialogSectionRenderer": {
                                    "title": {
                                        "runs": [{
                                            "text": "General"
                                        }]
                                    },
                                    "options": [{
                                        "hotkeyDialogSectionOptionRenderer": {
                                            "label": {
                                                "runs": [{
                                                    "text": "Toggle full screen"
                                                }]
                                            },
                                            "hotkey": "f"
                                        }
                                    }, {
                                        "hotkeyDialogSectionOptionRenderer": {
                                            "label": {
                                                "runs": [{
                                                    "text": "Toggle theater mode"
                                                }]
                                            },
                                            "hotkey": "t"
                                        }
                                    }, {
                                        "hotkeyDialogSectionOptionRenderer": {
                                            "label": {
                                                "runs": [{
                                                    "text": "Toggle miniplayer"
                                                }]
                                            },
                                            "hotkey": "i"
                                        }
                                    }, {
                                        "hotkeyDialogSectionOptionRenderer": {
                                            "label": {
                                                "runs": [{
                                                    "text": "Close miniplayer or current dialog"
                                                }]
                                            },
                                            "hotkey": "ESCAPE"
                                        }
                                    }, {
                                        "hotkeyDialogSectionOptionRenderer": {
                                            "label": {
                                                "runs": [{
                                                    "text": "Toggle mute"
                                                }]
                                            },
                                            "hotkey": "m"
                                        }
                                    }]
                                }
                            }, {
                                "hotkeyDialogSectionRenderer": {
                                    "title": {
                                        "runs": [{
                                            "text": "Subtitles and closed captions"
                                        }]
                                    },
                                    "options": [{
                                        "hotkeyDialogSectionOptionRenderer": {
                                            "label": {
                                                "runs": [{
                                                    "text": "If the video supports captions, toggle captions ON/OFF"
                                                }]
                                            },
                                            "hotkey": "c"
                                        }
                                    }, {
                                        "hotkeyDialogSectionOptionRenderer": {
                                            "label": {
                                                "runs": [{
                                                    "text": "Rotate through different text opacity levels"
                                                }]
                                            },
                                            "hotkey": "o"
                                        }
                                    }, {
                                        "hotkeyDialogSectionOptionRenderer": {
                                            "label": {
                                                "runs": [{
                                                    "text": "Rotate through different window opacity levels"
                                                }]
                                            },
                                            "hotkey": "w"
                                        }
                                    }, {
                                        "hotkeyDialogSectionOptionRenderer": {
                                            "label": {
                                                "runs": [{
                                                    "text": "Rotate through font sizes (increasing)"
                                                }]
                                            },
                                            "hotkey": "+"
                                        }
                                    }, {
                                        "hotkeyDialogSectionOptionRenderer": {
                                            "label": {
                                                "runs": [{
                                                    "text": "Rotate through font sizes (decreasing)"
                                                }]
                                            },
                                            "hotkey": "-",
                                            "hotkeyAccessibilityLabel": {
                                                "accessibilityData": {
                                                    "label": "Minus"
                                                }
                                            }
                                        }
                                    }]
                                }
                            }, {
                                "hotkeyDialogSectionRenderer": {
                                    "title": {
                                        "runs": [{
                                            "text": "Spherical Videos"
                                        }]
                                    },
                                    "options": [{
                                        "hotkeyDialogSectionOptionRenderer": {
                                            "label": {
                                                "runs": [{
                                                    "text": "Pan up"
                                                }]
                                            },
                                            "hotkey": "w"
                                        }
                                    }, {
                                        "hotkeyDialogSectionOptionRenderer": {
                                            "label": {
                                                "runs": [{
                                                    "text": "Pan left"
                                                }]
                                            },
                                            "hotkey": "a"
                                        }
                                    }, {
                                        "hotkeyDialogSectionOptionRenderer": {
                                            "label": {
                                                "runs": [{
                                                    "text": "Pan down"
                                                }]
                                            },
                                            "hotkey": "s"
                                        }
                                    }, {
                                        "hotkeyDialogSectionOptionRenderer": {
                                            "label": {
                                                "runs": [{
                                                    "text": "Pan right"
                                                }]
                                            },
                                            "hotkey": "d"
                                        }
                                    }, {
                                        "hotkeyDialogSectionOptionRenderer": {
                                            "label": {
                                                "runs": [{
                                                    "text": "Zoom in"
                                                }]
                                            },
                                            "hotkey": "+ on numpad or ]",
                                            "hotkeyAccessibilityLabel": {
                                                "accessibilityData": {
                                                    "label": "Plus on number pad or right bracket"
                                                }
                                            }
                                        }
                                    }, {
                                        "hotkeyDialogSectionOptionRenderer": {
                                            "label": {
                                                "runs": [{
                                                    "text": "Zoom out"
                                                }]
                                            },
                                            "hotkey": "- on numpad or [",
                                            "hotkeyAccessibilityLabel": {
                                                "accessibilityData": {
                                                    "label": "Minus on number pad or left bracket"
                                                }
                                            }
                                        }
                                    }]
                                }
                            }],
                            "dismissButton": {
                                "buttonRenderer": {
                                    "style": "STYLE_BLUE_TEXT",
                                    "size": "SIZE_DEFAULT",
                                    "isDisabled": false,
                                    "text": {
                                        "runs": [{
                                            "text": "Dismiss"
                                        }]
                                    },
                                    "trackingParams": "CAkQ8FsiEwjH2o6v6vH9AhVk50wCHY1yDbw="
                                }
                            },
                            "trackingParams": "CAgQteYDIhMIx9qOr-rx_QIVZOdMAh2Ncg28"
                        }
                    },
                    "backButton": {
                        "buttonRenderer": {
                            "trackingParams": "CAcQvIYDIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                            "command": {
                                "clickTrackingParams": "CAcQvIYDIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                "commandMetadata": {
                                    "webCommandMetadata": {
                                        "sendPost": true
                                    }
                                },
                                "signalServiceEndpoint": {
                                    "signal": "CLIENT_SIGNAL",
                                    "actions": [{
                                        "clickTrackingParams": "CAcQvIYDIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                        "signalAction": {
                                            "signal": "HISTORY_BACK"
                                        }
                                    }]
                                }
                            }
                        }
                    },
                    "forwardButton": {
                        "buttonRenderer": {
                            "trackingParams": "CAYQvYYDIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                            "command": {
                                "clickTrackingParams": "CAYQvYYDIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                "commandMetadata": {
                                    "webCommandMetadata": {
                                        "sendPost": true
                                    }
                                },
                                "signalServiceEndpoint": {
                                    "signal": "CLIENT_SIGNAL",
                                    "actions": [{
                                        "clickTrackingParams": "CAYQvYYDIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                        "signalAction": {
                                            "signal": "HISTORY_FORWARD"
                                        }
                                    }]
                                }
                            }
                        }
                    },
                    "a11ySkipNavigationButton": {
                        "buttonRenderer": {
                            "style": "STYLE_DEFAULT",
                            "size": "SIZE_DEFAULT",
                            "isDisabled": false,
                            "text": {
                                "runs": [{
                                    "text": "Skip navigation"
                                }]
                            },
                            "trackingParams": "CAUQ8FsiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                            "command": {
                                "clickTrackingParams": "CAUQ8FsiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                "commandMetadata": {
                                    "webCommandMetadata": {
                                        "sendPost": true
                                    }
                                },
                                "signalServiceEndpoint": {
                                    "signal": "CLIENT_SIGNAL",
                                    "actions": [{
                                        "clickTrackingParams": "CAUQ8FsiEwjH2o6v6vH9AhVk50wCHY1yDbw=",
                                        "signalAction": {
                                            "signal": "SKIP_NAVIGATION"
                                        }
                                    }]
                                }
                            }
                        }
                    },
                    "voiceSearchButton": {
                        "buttonRenderer": {
                            "style": "STYLE_DEFAULT",
                            "size": "SIZE_DEFAULT",
                            "isDisabled": false,
                            "serviceEndpoint": {
                                "clickTrackingParams": "CAIQ7a8FIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                "commandMetadata": {
                                    "webCommandMetadata": {
                                        "sendPost": true
                                    }
                                },
                                "signalServiceEndpoint": {
                                    "signal": "CLIENT_SIGNAL",
                                    "actions": [{
                                        "clickTrackingParams": "CAIQ7a8FIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                        "openPopupAction": {
                                            "popup": {
                                                "voiceSearchDialogRenderer": {
                                                    "placeholderHeader": {
                                                        "runs": [{
                                                            "text": "Listening..."
                                                        }]
                                                    },
                                                    "promptHeader": {
                                                        "runs": [{
                                                            "text": "Didn't hear that. Try again."
                                                        }]
                                                    },
                                                    "exampleQuery1": {
                                                        "runs": [{
                                                            "text": "\"Play Dua Lipa\""
                                                        }]
                                                    },
                                                    "exampleQuery2": {
                                                        "runs": [{
                                                            "text": "\"Show me my subscriptions\""
                                                        }]
                                                    },
                                                    "promptMicrophoneLabel": {
                                                        "runs": [{
                                                            "text": "Tap microphone to try again"
                                                        }]
                                                    },
                                                    "loadingHeader": {
                                                        "runs": [{
                                                            "text": "Working..."
                                                        }]
                                                    },
                                                    "connectionErrorHeader": {
                                                        "runs": [{
                                                            "text": "No connection"
                                                        }]
                                                    },
                                                    "connectionErrorMicrophoneLabel": {
                                                        "runs": [{
                                                            "text": "Check your connection and try again"
                                                        }]
                                                    },
                                                    "permissionsHeader": {
                                                        "runs": [{
                                                            "text": "Waiting for permission"
                                                        }]
                                                    },
                                                    "permissionsSubtext": {
                                                        "runs": [{
                                                            "text": "Allow microphone access to search with voice"
                                                        }]
                                                    },
                                                    "disabledHeader": {
                                                        "runs": [{
                                                            "text": "Search with your voice"
                                                        }]
                                                    },
                                                    "disabledSubtext": {
                                                        "runs": [{
                                                            "text": "To search by voice, go to your browser settings and allow access to microphone"
                                                        }]
                                                    },
                                                    "microphoneButtonAriaLabel": {
                                                        "runs": [{
                                                            "text": "Cancel"
                                                        }]
                                                    },
                                                    "exitButton": {
                                                        "buttonRenderer": {
                                                            "style": "STYLE_DEFAULT",
                                                            "size": "SIZE_DEFAULT",
                                                            "isDisabled": false,
                                                            "icon": {
                                                                "iconType": "CLOSE"
                                                            },
                                                            "trackingParams": "CAQQ0LEFIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                            "accessibilityData": {
                                                                "accessibilityData": {
                                                                    "label": "Cancel"
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "trackingParams": "CAMQ7q8FIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                                                    "microphoneOffPromptHeader": {
                                                        "runs": [{
                                                            "text": "Microphone off. Try again."
                                                        }]
                                                    }
                                                }
                                            },
                                            "popupType": "TOP_ALIGNED_DIALOG"
                                        }
                                    }]
                                }
                            },
                            "icon": {
                                "iconType": "MICROPHONE_ON"
                            },
                            "tooltip": "Search with your voice",
                            "trackingParams": "CAIQ7a8FIhMIx9qOr-rx_QIVZOdMAh2Ncg28",
                            "accessibilityData": {
                                "accessibilityData": {
                                    "label": "Search with your voice"
                                }
                            }
                        }
                    }
                }
            },
            "microformat": {
                "microformatDataRenderer": {
                    "urlCanonical": "https://www.youtube.com/channel/UCmjdhwMGSut8mZ1CqnRjjUw",
                    "title": "麦兜搞IT",
                    "description": "Welcome to my channel, this Channel focus on technology like networking, automation, Python, Docker, kubernetes, git, etc. 大家好，欢迎大家来到我的频道，我目前在荷兰海牙工作。 B站账户麦兜搞...",
                    "thumbnail": {
                        "thumbnails": [{
                            "url": "https://yt3.googleusercontent.com/ytc/AL5GRJVEHo8nD-_MA7lwd7BvXCsV09Huj--2dCo59a0=s200-c-k-c0x00ffffff-no-rj?days_since_epoch=19439",
                            "width": 200,
                            "height": 200
                        }]
                    },
                    "siteName": "YouTube",
                    "appName": "YouTube",
                    "androidPackage": "com.google.android.youtube",
                    "iosAppStoreId": "544007664",
                    "iosAppArguments": "https://www.youtube.com/channel/UCmjdhwMGSut8mZ1CqnRjjUw",
                    "ogType": "yt-fb-app:channel",
                    "urlApplinksWeb": "https://www.youtube.com/channel/UCmjdhwMGSut8mZ1CqnRjjUw?feature=applinks",
                    "urlApplinksIos": "vnd.youtube://www.youtube.com/channel/UCmjdhwMGSut8mZ1CqnRjjUw?feature=applinks",
                    "urlApplinksAndroid": "vnd.youtube://www.youtube.com/channel/UCmjdhwMGSut8mZ1CqnRjjUw?feature=applinks",
                    "urlTwitterIos": "vnd.youtube://www.youtube.com/channel/UCmjdhwMGSut8mZ1CqnRjjUw?feature=twitter-deep-link",
                    "urlTwitterAndroid": "vnd.youtube://www.youtube.com/channel/UCmjdhwMGSut8mZ1CqnRjjUw?feature=twitter-deep-link",
                    "twitterCardType": "summary",
                    "twitterSiteHandle": "@YouTube",
                    "schemaDotOrgType": "http://schema.org/http://schema.org/YoutubeChannelV2",
                    "noindex": false,
                    "unlisted": false,
                    "familySafe": true,
                    "tags": ["Technology"],
                    "availableCountries": ["CR", "SV", "SS", "VC", "GA", "ID", "LB", "MT", "IE", "NF", "GG", "MX", "AD", "HR", "KW", "NE", "TF", "FO", "BG", "FM", "EH", "AU", "LS", "SJ", "UY", "SD", "UA", "AW", "EC", "TO", "TH", "GI", "AI", "LC", "MH", "VA", "IO", "PN", "NZ", "AE", "SB", "IS", "FI", "ES", "BL", "CW", "GF", "TK", "MO", "EE", "TN", "PM", "TW", "MW", "GB", "PL", "UM", "JP", "IQ", "MR", "RO", "SX", "AS", "HT", "JM", "ET", "ER", "NC", "FJ", "BI", "PW", "LU", "VN", "TD", "CA", "CN", "RW", "OM", "TL", "VU", "SE", "SG", "KR", "GH", "PG", "TJ", "PE", "SA", "MQ", "PR", "PS", "SK", "IR", "BQ", "CV", "LV", "MA", "AR", "RE", "VI", "CZ", "MK", "GS", "MV", "PF", "NL", "RU", "GT", "AL", "BH", "BR", "BT", "TR", "CF", "UG", "UZ", "ML", "CU", "HM", "PK", "SI", "SO", "TG", "DM", "LK", "AM", "AT", "JO", "BB", "LR", "SR", "ZM", "CL", "PT", "FR", "BW", "AO", "ZA", "BE", "GM", "AF", "GN", "NR", "DJ", "IN", "KE", "IT", "PH", "PY", "BY", "LA", "DO", "YT", "LY", "JE", "CX", "KH", "GR", "BN", "MZ", "BS", "CO", "AZ", "NA", "SY", "BD", "VG", "SZ", "HU", "HK", "KY", "WF", "KG", "MP", "TZ", "NG", "KZ", "ZW", "GD", "BV", "CI", "RS", "CY", "QA", "LI", "YE", "IL", "SM", "CK", "NP", "SH", "AX", "AG", "MC", "SL", "CM", "CG", "MG", "GQ", "BM", "DE", "NI", "AQ", "ME", "NO", "PA", "GU", "MY", "CD", "BO", "MM", "SC", "KM", "TT", "VE", "TM", "NU", "MN", "MF", "HN", "LT", "BZ", "FK", "BA", "MD", "ST", "BJ", "EG", "WS", "TV", "MS", "US", "DZ", "GE", "BF", "SN", "GY", "IM", "GP", "KI", "CH", "DK", "KP", "GL", "GW", "CC", "MU", "TC", "KN"],
                    "linkAlternates": [{
                        "hrefUrl": "https://m.youtube.com/channel/UCmjdhwMGSut8mZ1CqnRjjUw"
                    }, {
                        "hrefUrl": "android-app://com.google.android.youtube/http/youtube.com/channel/UCmjdhwMGSut8mZ1CqnRjjUw"
                    }, {
                        "hrefUrl": "ios-app://544007664/http/youtube.com/channel/UCmjdhwMGSut8mZ1CqnRjjUw"
                    }]
                }
            }
        };"#;
        // let data = "key1=value1\nkey2=value2\nkey3=value3\n";
        // let expected = vec![
        //     ("key1", "value1"),
        //     ("key2", "value2"),
        //     ("key3", "value3"),
        // ]
        // .into_iter()
        // .collect::<HashMap<&str, &str>>();
        // assert_eq!(parse_data(data), expected);


        // for g in parse_data(data){
        //     print!("{}---{}",g.0,g.1)
        // }
        let mut store = String::new();
    for (key, value) in parse_data(data) {
        store.push_str(&format!("{}={}\n", key, value));
    }
    std::fs::write("./try.txt", store).unwrap();
    }
}
