MARP=marp --theme $(ASSET_DIR)/custom.css

TOP_DIR=.
SRC_DIR=$(TOP_DIR)/slides
TARGET_DIR=$(TOP_DIR)/_build
ASSET_DIR=$(TOP_DIR)/assets

run:
	@$(MARP) -s $(SRC_DIR) --html

build: $(TARGET_DIR) copy-assets
	@$(MARP) -I $(SRC_DIR) -o _build

build-pdf: build
	@$(MARP) -I $(SRC_DIR) -o _build --allow-local-files --pdf --html

copy-assets:
	@rsync -arv $(SRC_DIR)/images $(TARGET_DIR)

$(TARGET_DIR):
	@mkdir -p $@
