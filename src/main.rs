// Barely Modified from the Original C# code. None of the code has been modified to be idiomatic
// Rust, but rather the most direct translation possible that still compiles.

extern crate gilded_rose;

pub use gilded_rose::goblin::Item;

use std::cmp;

pub trait ItemUpdater {
    fn update_item(&self, item: &Item) -> Item;
}

pub struct DefaultItemUpdater;
impl ItemUpdater for DefaultItemUpdater {
    fn update_item(&self, item: &Item) -> Item {
        if item.sell_in > 0 {
            Item{ name: item.name, sell_in: item.sell_in - 1, quality: cmp::max(item.quality - 1, 0)}
        } else {
            Item{ name: item.name, sell_in: item.sell_in - 1, quality: cmp::max(item.quality - 2, 0)}
        }
    }
}

pub struct AgedItemUpdater;
impl ItemUpdater for AgedItemUpdater {
    fn update_item(&self, item: &Item) -> Item {
        Item{ name: item.name, sell_in: item.sell_in - 1, quality: cmp::min(item.quality + 1, 50)}
    }
}

fn main() {
    let mut items = vec!
    {
        Item { name: "+5 Dexterity Vest", sell_in: 10, quality: 20 },
        Item { name: "Aged Brie", sell_in: 2, quality: 0 },
        Item { name: "Elixir of the Mongoose", sell_in: 5, quality: 7 },
        Item { name: "Sulfuras, Hand of Ragnaros", sell_in: 0, quality: 80 },
        Item { name: "Backstage passes to a TAFKAL80ETC concert", sell_in: 15, quality: 20 },
        Item { name: "Conjured Mana Cake", sell_in: 3, quality: 6 }
    };

    for i in 0..50 {
        println!("Day {}:\n========================================", i);
        for item in &items {
            println!("{:?}", item);
        }
        items = update_quality(&mut items[..]);
    }
}

fn update_quality(items: &mut [Item]) -> Vec<Item>
{
    items.iter().map(|item: &Item| get_updater(item.name).update_item(item)).collect()
}

fn get_updater(s: &str) -> Box<ItemUpdater> {
    if s == "Aged Brie" {
        Box::new(AgedItemUpdater)
    } else {
        Box::new(DefaultItemUpdater)
    }
}


/*

    for i in 0..items.len()
    {
        if (items[i].name != "Aged Brie" && items[i].name != "Backstage passes to a TAFKAL80ETC concert")
        {
            if (items[i].quality > 0)
            {
                if (items[i].name != "Sulfuras, Hand of Ragnaros")
                {
                    items[i].quality = items[i].quality - 1;
                }
            }
        }
        else
        {
            if (items[i].quality < 50)
            {
                items[i].quality = items[i].quality + 1;

                if (items[i].name == "Backstage passes to a TAFKAL80ETC concert")
                {
                    if (items[i].sell_in < 11)
                    {
                        if (items[i].quality < 50)
                        {
                            items[i].quality = items[i].quality + 1;
                        }
                    }

                    if (items[i].sell_in < 6)
                    {
                        if (items[i].quality < 50)
                        {
                            items[i].quality = items[i].quality + 1;
                        }
                    }
                }
            }
        }

        if (items[i].name != "Sulfuras, Hand of Ragnaros")
        {
            items[i].sell_in = items[i].sell_in - 1;
        }

        if (items[i].sell_in < 0)
        {
            if (items[i].name != "Aged Brie")
            {
                if (items[i].name != "Backstage passes to a TAFKAL80ETC concert")
                {
                    if (items[i].quality > 0)
                    {
                        if (items[i].name != "Sulfuras, Hand of Ragnaros")
                        {
                            items[i].quality = items[i].quality - 1;
                        }
                    }
                }
                else
                {
                    items[i].quality = items[i].quality - items[i].quality;
                }
            }
            else
            {
                if (items[i].quality < 50)
{
                    items[i].quality = items[i].quality + 1;
                }
            }
        }
    }
    */

#[test]
fn normal_items_decrease_quality() {
    let mut items = vec![
    Item { name: "+5 Dexterity Vest", sell_in: 10, quality: 20 },
    ];
    let results = update_quality(&mut items[..]);
    assert_eq!(results[0].sell_in, 9);
    assert_eq!(results[0].quality, 19);
}

#[test]
fn quality_degrades_twice_as_fast_after_sellby_date() {
    let mut items = vec![
    Item { name: "+5 Dexterity Vest", sell_in: 0, quality: 20 },
    ];
    let results = update_quality(&mut items[..]);
    assert_eq!(results[0].sell_in, -1);
    assert_eq!(results[0].quality, 18);
}

#[test]
fn quality_never_goes_below_zero() {
    let mut items = vec![
    Item { name: "+5 Dexterity Vest", sell_in: 0, quality: 1 },
    ];
    let results = update_quality(&mut items[..]);
    assert_eq!(results[0].sell_in, -1);
    assert_eq!(results[0].quality, 0);
}

#[test]
fn aged_brie_increases_quality_as_it_ages() {
    let mut items = vec![
    Item { name: "Aged Brie", sell_in: 10, quality: 1 },
    ];
    let results = update_quality(&mut items[..]);
    assert_eq!(results[0].sell_in, 9);
    assert_eq!(results[0].quality, 2);
}

#[test]
fn aged_brie_quality_never_exceedes_50() {
    let mut items = vec![
    Item { name: "Aged Brie", sell_in: 10, quality: 49 },
    ];

    let mut intermediate_results = update_quality(&mut items[..]);
    let results = update_quality(&mut intermediate_results[..]);
    assert_eq!(results[0].sell_in, 8);
    assert_eq!(results[0].quality, 50);
}

#[test]
fn sulfuras_never_changes_sell_in_or_quality() {
    let mut items = vec![
    Item { name: "Sulfuras, Hand of Ragnaros", sell_in: 10, quality: 49 },
    ];
    let results = update_quality(&mut items[..]);
    assert_eq!(results[0].sell_in, 10);
    assert_eq!(results[0].quality, 49);
}

#[test]
fn backstage_passes_increases_in_quality_by_1_if_sellin_over_10() {
    let mut items = vec![
    Item { name: "Backstage passes to a TAFKAL80ETC concert", sell_in: 11, quality: 49 },
    ];
    let results = update_quality(&mut items[..]);
    assert_eq!(results[0].sell_in, 10);
    assert_eq!(results[0].quality, 50);
}

#[test]
fn backstage_passes_increases_in_quality_by_2_if_sellin_under_11_and_over_5() {
    let mut items = vec![
    Item { name: "Backstage passes to a TAFKAL80ETC concert", sell_in: 10, quality: 10 },
    ];
    let results = update_quality(&mut items[..]);
    assert_eq!(results[0].sell_in, 9);
    assert_eq!(results[0].quality, 12);
}

#[test]
fn backstage_passes_increases_in_quality_by_2_if_sellin_under_6_and_over_0() {
    let mut items = vec![
    Item { name: "Backstage passes to a TAFKAL80ETC concert", sell_in: 5, quality: 10 },
    ];
    let results = update_quality(&mut items[..]);
    assert_eq!(results[0].sell_in, 4);
    assert_eq!(results[0].quality, 13);
}

#[test]
fn backstage_passes_quality_goes_to_zero_if_sellin_is_0() {
    let mut items = vec![
    Item { name: "Backstage passes to a TAFKAL80ETC concert", sell_in: 0, quality: 10 },
    ];
    let results = update_quality(&mut items[..]);
    assert_eq!(results[0].sell_in, -1);
    assert_eq!(results[0].quality, 0);
}
