SELECT
  i.*,
  ic.label AS class_label, ic.label_frFR AS class_label_frFR,
  isc.label AS subclass_label, isc.label_frFR AS subclass_label_frFR,
  iq.label AS quality_label, iq.label_frFR AS quality_label_frFR,
  islot.label AS slot_label, islot.label_frFR AS slot_label_frFR,
  src.name AS source_name, src.cost, src.zone_id, src.drop_chance,
  z.name AS zone_name, z.level_min AS zone_level_min, z.level_max AS zone_level_max,
  sc.label AS source_category_label, sc.label_frFR AS source_category_label_frFR,
  f.label AS source_faction_label, f.label_frFR AS source_faction_label_frFR
FROM item i
LEFT JOIN item_class ic ON i.class_id = ic.id
LEFT JOIN item_subclass isc ON i.subclass_id = isc.id
LEFT JOIN item_quality iq ON i.quality_id = iq.id
LEFT JOIN item_slot islot ON i.slot_id = islot.id
LEFT JOIN item_source src ON i.source_id = src.id
LEFT JOIN zone z ON src.zone_id = z.id
LEFT JOIN item_source_category sc ON src.category_id = sc.id
LEFT JOIN faction f ON src.faction_id = f.id
WHERE i.item_id = 1913;

SELECT
  t.label AS tooltip_label,
  tf.label AS tooltip_format_label
FROM item_tooltip_link l
JOIN item_tooltip t ON l.tooltip_id = t.id
LEFT JOIN item_tooltip_format tf ON t.format_id = tf.id
WHERE l.item_id = 1913;